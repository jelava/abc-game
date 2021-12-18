const userId = sessionStorage.getItem('userId');

document.addEventListener('DOMContentLoaded', () => {
    fetch('/games', { method: 'GET' })
        .then(response => response.json())
        .then(json => json.games.forEach(addGameToList))
        .catch(console.error);
    
    joinLobby();
});

const addGameToList = game => {
    const button = document.createElement('button');
    button.appendChild(document.createTextNode('Join'));
    button.addEventListener('click', onJoinGame.bind(window, game.game_id));

    const gameDescription = `#${game.game_id} – host: ${game.host_name}, players: ${game.player_count}`;

    const listItem = document.createElement('li');
    listItem.setAttribute('gameId', game.game_id);
    listItem.appendChild(document.createTextNode(gameDescription))
    listItem.appendChild(button);

    document.getElementById('gameList')
        .appendChild(listItem);
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
    const eventSource = new EventSource(`/lobby/join/${userId}`);
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

const onJoinGame = gameId => {
    fetch(`/lobby/leave/${userId}`, { method: 'POST' })
        .catch(console.error)
        .finally(() => {
            sessionStorage.setItem('gameId', gameId);
            location.pathname = 'wait.html';        
        });
};

const onHost = () => {
    fetch(`/games/host/${userId}`, { method: 'POST' })
        .then(response => response.json())
        .then(json => {
            console.warn(json);
            onJoinGame(json.game_id);
        })
        .catch(console.error);
};
