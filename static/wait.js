document.addEventListener('DOMContentLoaded', () => {
    initPage();

    const userId = sessionStorage.getItem('userId');
    const eventSource = new EventSource(`/users/${userId}/events`);
    eventSource.addEventListener('startGame', onStartGame);

    const gameId = sessionStorage.getItem('gameId');
    fetch(`/games/${gameId}/join/${userId}`, { method: 'PATCH' });
});

const initPage = () => {
    const gameName = sessionStorage.getItem('gameName');
    const hostName = sessionStorage.getItem('hostName');

    const gameNameDisplay = document.getElementById('gameName');
    gameNameDisplay.appendChild(document.createTextNode(gameName));

    const hostNameDisplay = document.getElementById('hostName');
    hostNameDisplay.appendChild(document.createTextNode(hostName));

    const isHost = sessionStorage.getItem('isHost');
    const hiddenClass = isHost ? 'playerOnly' : 'hostOnly';

    for (const element of document.getElementsByClassName(hiddenClass)) {
        element.style.display = 'none';
    }
};

const onStartGame = event => {
    console.log(event);

    const initials = event.data.split(' ');
    console.log(initials);

    alert('TODO: store initials and got to play.html');
};

const onHostStart = () => {
    alert('TODO: /games/${gameId}/start');
};

const onCancel = () => {
    alert('TODO: close eventSource and send request to remove player from game');
};
