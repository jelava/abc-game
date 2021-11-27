document.addEventListener('DOMContentLoaded', () => {
    fetch('/games', { method: 'GET' })
        .then(response => response.json())
        .then(json => json.games)
        .then(fillGameList)
        .then(connectToEvents)
        .catch(console.error);
});

const fillGameList = games => {
    const gameList = document.getElementById('gameList');

    if (games.length > 0) {
        for (const game of games) {
            const button = document.createElement('button');
            button.appendChild(document.createTextNode('Join'));
            button.addEventListener('click', onJoinGame(game.game_id, game.name, game.host_name));

            const gameDescription = `${game.name} – host: ${game.host_name}, players: ${game.player_count}`;

            const listItem = document.createElement('li');
            listItem.setAttribute('gameId', game.game_id);
            listItem.appendChild(document.createTextNode(gameDescription))
            listItem.appendChild(button);

            gameList.appendChild(listItem);
        }
    } else {
        const listItem = document.createElement('li');
        listItem.appendChild(document.createTextNode('There are no open games. Press the button below to host a new game, or try refreshing the page to see if a new game has opened.'));
        gameList.appendChild(listItem);
    }
};

const connectToEvents = () => {
    const userId = sessionStorage.getItem('userId');
    const eventSource = new EventSource(`/users/${userId}/events`);
    eventSource.addEventListener('sseTest', event => console.log('sse test success'));
    eventSource.addEventListener('gameOpened', onGameOpened);
    eventSource.addEventListener('gameClosed', onGameClosed);
};

const onGameOpened = event => {
    const gameInfo = JSON.parse(event.data);
    const gameList = document.getElementById('gameList');
};

const onGameClosed = event => {
    const gameId = event.data;
};

const onJoinGame = (gameId, gameName, hostName) => async () => {
    sessionStorage.setItem('gameId', gameId);
    sessionStorage.setItem('gameName', gameName);
    sessionStorage.setItem('hostName', hostName);
    location.pathname = 'wait.html';
};

const onHost = () => {
    alert('TODO!');
};
