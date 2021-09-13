document.addEventListener('DOMContentLoaded', () => {
    // TODO: error handling?
    fetch('/games', { method: 'GET' })
        .then(response => response.json())
        .then(json => json.games)
        .then(fillGameList);
});

const fillGameList = games => {
    const gameList = document.getElementById('gameList');

    if (games.length > 0) {
        for (const [gameId, game] of games.entries()) {
            const button = document.createElement('button');
            button.appendChild(document.createTextNode('Join'));
            button.addEventListener('click', onJoinGame(gameId));

            const gameDescription = `${game.name} – host: ${game.host_name}, players: ${game.player_count}`;

            const listItem = document.createElement('li');
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

const onJoinGame = gameId => async () => {
    const userId = sessionStorage.getItem('userId');

    const initials = await fetch(`/games/${gameId}/join/${userId}`, { method: 'PATCH' })
        .then(response => response.json())
        .then(json => json.initials);

    sessionStorage.setItem('gameId', gameId);
    location.pathname = 'play.html';
};

const onHost = () => {
    alert('TODO!');
}
