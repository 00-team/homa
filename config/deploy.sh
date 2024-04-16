
SPACER="======================================"
EG="🔷"

cd /thora/

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

if check_diff "config/*.service"; then
    echo "$EG reload the services"
    cp config/*.service /etc/systemd/system/ --force
    systemctl daemon-reload
    echo $SPACER
fi

if [ ! -f web/main.db ] || check_diff "web/migrations/*"; then
    echo "$EG setup the web database"
    export $(head -n 1 web/.secrets.env | xargs)
    cd web ; cargo sqlx database setup ; cd ..
    echo $SPACER
fi

if [ ! -f bot/main.db ] || check_diff "bot/migrations/*"; then
    echo "$EG setup the bot database"
    export $(head -n 1 bot/.secrets.env | xargs)
    cd bot ; cargo sqlx database setup ; cd ..
    echo $SPACER
fi

if check_diff "bot/*"; then
    echo "$EG cargo build bot"
    cargo build -r -p bot
    systemctl restart thora.bot
    echo $SPACER
fi

if check_diff "web/*"; then
    echo "$EG cargo build bot"
    cargo build -r -p web
    systemctl restart thora.web
    echo $SPACER
fi

echo "Deploy is Done! ✅"

