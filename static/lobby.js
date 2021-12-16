const userId = sessionStorage.getItem('userId');

document.addEventListener('DOMContentLoaded', () => {
    fetch('/games', { method: 'GET' })
        .then(response => response.json())
        .then(json => json.games.forEach(addGameToList))
        .catch(console.error);
    
    joinLobby();
});

const addGameToList = game => {
    const gameList = document.getElementById('gameList');

    const button = document.createElement('button');
    button.appendChild(document.createTextNode('Join'));
    button.addEventListener('click', onJoinGame(game.game_id, game.name, game.host_name));

    const gameDescription = `${game.name} – host: ${game.host_name}, players: ${game.player_count}`;

    const listItem = document.createElement('li');
    listItem.setAttribute('gameId', game.game_id);
    listItem.appendChild(document.createTextNode(gameDescription))
    listItem.appendChild(button);

    gameList.appendChild(listItem);
};

const removeGameFromList = gameId => {
    const gameList = document.getElementById('gameList');
    const listItem = gameList.querySelector(`li[gameId=${gameId}]`);

    console.info(listItem);
    
    if (listItem) {
        gameList.removeChild(listItem);
    } else {
        console.error(`Could not find any game with ID: ${gameId}`);
    }
};

const joinLobby = () => {
    const eventSource = new EventSource(`/lobby`);
    eventSource.addEventListener('gameOpened', onGameOpened);
    eventSource.addEventListener('gameClosed', onGameClosed);
};

const onGameOpened = event => {
    const gameInfo = JSON.parse(event.data);
    addGameToList(gameInfo);
};

const onGameClosed = event => {
    const gameId = event.data;
    console.warn('TODO');
    console.warn(gameId);
};

const onJoinGame = (gameId, gameName, hostName) => async () => {
    sessionStorage.setItem('gameId', gameId);
    //sessionStorage.setItem('gameName', gameName);
    //sessionStorage.setItem('hostName', hostName);
    location.pathname = 'wait.html';
};

const onHost = () => {
    fetch(`/games/host/${userId}`, { method: 'POST' })
        .then(response => response.json())
        .then(json => {
            sessionStorage.setItem('gameId');
            location.pathname('wait.html');
        })
        .catch(console.error);
};
