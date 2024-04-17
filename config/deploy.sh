
SPACER="======================================"
EG="🔷"

cd /thora/
export $(head -n 1 .secrets.env | xargs) # DATABASE_URL

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

echo "🧹 removing the teloxide database"
rm -rf /thora/bot/teloxide.db
echo $SPACER

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
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build web"
    cargo build -r
    systemctl restart thora.web
    echo $SPACER
fi

cd ../bot
if [ ! -f bot/main.db ] || check_diff "migrations/*"; then
    echo "$EG setup the bot database"
    cargo sqlx database setup
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build bot"
    cargo build -r
    systemctl restart thora.bot
    echo $SPACER
fi


echo "Deploy is Done! ✅"

