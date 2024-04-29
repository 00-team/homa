
SPACER="======================================"
EG="🔷"

cd /thora/
export $(cat .secrets.env | xargs)

OLD_COMMIT=$(git rev-parse HEAD)

echo "$EG update the source"
git pull
echo $SPACER

NEW_COMMIT=$(git rev-parse HEAD)

function check_diff {
    local file_has_changed=$(git diff --name-only $OLD_COMMIT...$NEW_COMMIT --exit-code $1)
    if [ -z "$file_has_changed" ]; then
        return 1
    else
        return 0
    fi
}

function send_message {
    sleep 2
    base_url="https://api.telegram.org/bot$TELOXIDE_TOKEN/sendMessage"
    curl -s "$base_url?chat_id=$TELOXIDE_DEVELOPER&text=$1" -o /dev/null &
}

function check_status {
    systemctl status $1 --no-pager --no-legend > /dev/null
    [[ $? = 0 ]] && e="✅" || e="❌"
    send_message "$1 status: $e"
}

if check_diff "config/*.service"; then
    echo "$EG reload the services"
    cp config/*.service /etc/systemd/system/ --force
    systemctl daemon-reload
    echo $SPACER
fi

if check_diff "app/* package.json"; then
    echo "$EG build the app"
    npm i
    npm run build
    echo $SPACER
fi

cd web
if [ ! -f main.db ] || check_diff "migrations/*"; then
    send_message "web db backup starting ⏳"
    mkdir -p backup
    tar czf backup/$(date +%s).tgz main.db
    send_message "web db backup done ⌛"

    rm main.db
    echo "$EG setup the web database"
    cargo sqlx db setup
    [[ $? = 0 ]] && e="✅" || e="❌"
    send_message "web db setup: $e"
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build web"
    send_message "building web"
    DATABASE_URL=sqlite://web/main.db cargo build -r
    [[ $? = 0 ]] && e="✅" || e="❌"
    send_message "web build status: $e"
    systemctl restart thora.web
    check_status thora.web
    echo $SPACER
fi

cd ../bot
if [ ! -f main.db ] || check_diff "migrations/*"; then
    echo "$EG setup the bot database"
    send_message "bot db backup starting ⏳"
    mkdir -p backup
    tar czf backup/$(date +%s).tgz main.db
    send_message "bot db backup done ⌛"

    rm main.db
    cargo sqlx db setup
    [[ $? = 0 ]] && e="✅" || e="❌"
    send_message "bot db setup: $e"
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build bot"
    send_message "building bot"
    DATABASE_URL=sqlite://bot/main.db cargo build -r
    [[ $? = 0 ]] && e="✅" || e="❌"
    send_message "bot build status: $e"
    echo $SPACER

    echo "🧹 removing the teloxide database"
    rm -rf /thora/bot/teloxide.db
    echo $SPACER

    echo "🔥 restart thora bot"
    systemctl restart thora.bot
    check_status thora.bot
    echo $SPACER
fi


send_message "Done: 🌩"
echo "Deploy is Done! ✅"

