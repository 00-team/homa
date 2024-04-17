
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
    curl "$base_url?chat_id=$TELOXIDE_DEVELOPER&text=$1" \ 
        1> /dev/null 2> /dev/null &
}

function check_status {
    systemctl status $1 --no-pager --no-legend > /dev/null
    send_message "$1 status: $($? == 0 ? ✅ : ❌)"
}

if check_diff "config/*.service"; then
    echo "$EG reload the services"
    cp config/*.service /etc/systemd/system/ --force
    systemctl daemon-reload
    echo $SPACER
fi

cd web
if [ ! -f web/main.db ] || check_diff "migrations/*"; then
    echo "$EG setup the web database"
    cargo sqlx database setup
    send_message "web db setup: $($? == 0 ? ✅ : ❌)"
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build web"
    send_message "building web"
    cargo build -r
    systemctl restart thora.web
    check_status thora.web
    echo $SPACER
fi

cd ../bot
if [ ! -f bot/main.db ] || check_diff "migrations/*"; then
    echo "$EG setup the bot database"
    cargo sqlx database setup
    send_message "bot db setup: $($? == 0 ? ✅ : ❌)"
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build bot"
    send_message "building bot"
    cargo build -r
    echo $SPACER

    echo "🧹 removing the teloxide database"
    rm -rf /thora/bot/teloxide.db
    echo $SPACER

    echo "🔥 restart thora bot"
    systemctl restart thora.bot
    check_status thora.bot
    echo $SPACER
fi


send_message "🌩"
echo "Deploy is Done! ✅"

