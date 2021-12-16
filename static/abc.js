/*******************
 * Creating a user *
 *******************/

 const adjectives = [
    'ambitious',
    'belching',
    'chaotic',
    'distinctive',
    'elusive',
    'flatulent',
    'gaseous',
    'humorous',
    'irritating',
    'jaunty',
    'kaleidoscopic',
    'loyal',
    'microscopic',
    'noisy',
    'odious',
    'peaceful',
    'quick',
    'royal',
    'smelly',
    'terrific',
    'uncanny',
    'vast',
    'wispy',
    'xyloid',
    'yawning',
    'zealous'
];

const animals = [
    'axolotl',
    'bumblebee',
    'chipmunk',
    'dodo',
    'elephant',
    'fox',
    'gorilla',
    'hedgehog',
    'ibex',
    'jellyfish',
    'krill',
    'lemur',
    'moose',
    'narwhal',
    'okapi',
    'porpoise',
    'quokka',
    'robin',
    'skunk',
    'termite',
    'uakari',
    'vole',
    'wildebeest',
    'xenoceratops',
    'yak',
    'zebu'
];

let userId;

document.addEventListener('DOMContentLoaded', () => {
    userId = sessionStorage.getItem('userId');
    const oldUserName = localStorage.getItem('userName');

    if (userId) {
        showLobby();
    } else if (oldUserName) {
        document.getElementById('userNameInput').value = oldUserName;
    } else {
        onRandomizeName();
    }
});

const onRandomizeName = () => {
    const r1 = Math.floor(Math.random() * adjectives.length);
    const r2 = Math.floor(Math.random() * animals.length);
    document.getElementById('userNameInput').value = `${adjectives[r1]} ${animals[r2]}`;
};

const onCreateUser = () => {
    const userName = document.getElementById('userNameInput').value;

    if (userName.length >= 3) {
        localStorage.setItem('userName', userName);
        const eventSource = new EventSource(`/users/${userName}`);
        eventSource.addEventListener('userCreated', onUserCreated);
    } else {
        alert('Please choose a nickname with at least three letters.');
    }
};

const onUserCreated = event => {
    userId = parseInt(event.data, 10);
    sessionStorage.setItem('userId', userId);
    showLobby();
};

/*************************
 * Displaying open games *
 *************************/

let gameId;
 
const showLobby = () => {
    document.getElementById('createUser').style.display = 'none';

    gameId = sessionStorage.getItem('gameId');

    if (gameId) {
        onJoinGame(gameId)();
    }

    document.getElementById('hostOrConnect').style.display = 'revert';

    fetch('/games', { method: 'GET' })
        .then(response => response.json())
        .then(json => json.games)
        .then(fillGameList)
        .catch(console.error);
};


const fillGameList = games => {
    const gameList = document.getElementById('gameList');

    if (games.length > 0) {
        for (const [gameId, game] of games.entries()) {
            let gameDescription = `${game.name} – host: ${game.host_name}, players: ${game.player_count}`;

            const link = document.createElement('a');
            link.setAttribute('href', `#game/${gameId}`);
            link.addEventListener('click', onJoinGame(gameId));
            link.appendChild(document.createTextNode(gameDescription));

            const listItem = document.createElement('li');
            listItem.appendChild(link);
            gameList.appendChild(listItem);
        }
    } else {
        const listItem = document.createElement('li');
        listItem.appendChild(document.createTextNode('There are no open games. Press the button below to host a new game, or try refreshing the page to see if a new game has opened.'));
        gameList.appendChild(listItem);
    }
};

const onJoinGame = selectedGameId => async () => {
    gameId = selectedGameId;

    const initials = await fetch(`/games/${gameId}/join/${userId}`, { method: 'PATCH' })
        .then(response => response.json())
        .then(json => json.initials);

    console.log(initials);

    document.getElementById('hostOrConnect').style.display = 'none';
    startGame(initials);
};

/******************
 * Hosting a game *
 ******************/

const onHost = () => {
    document.getElementById('hostOrConnect').style.display = 'none';
    document.getElementById('hostGame').style.display = 'revert';
};

const onHostCancel = () => {
    console.log('host canceled');
    document.getElementById('hostGame').style.display = 'none';
    document.getElementById('hostOrConnect').style.display = 'revert';
};

const onHostStart = () => {
    document.getElementById('hostGame').style.display = 'none';
    console.warn('TODO!')
    startGame();
};

/************************
 * Connecting to a game *
 ************************/

const onConnect = event => {
    document.getElementById('hostOrConnect').style.display = 'none';
    document.getElementById('connectToGame').style.display = 'revert';
};

const onReady = event => {
    console.log('ready');
};

const onCancel = event => {
    console.log('cancel');
    document.getElementById('connectToGame').style.display = 'none';
    document.getElementById('hostOrConnect').style.display = 'revert';
};

/*************
 * Main game *
 *************/

const timeLimit = 1000 * 60 * 0.5;
let timerEnd = Date.now(); 

const startGame = initials => {
    document.getElementById('mainGame').style.display = 'revert';

    // create N-1 copies of the nameList div and sets initials for each one
    const mainGameDiv = document.getElementById('mainGame')
    const nameListDiv = mainGameDiv.querySelector('.nameList');
    nameListDiv.querySelector('.initials').innerHTML = initials[0];

    for (let i = 1; i < initials.length; i++) {
        const clonedNameListDiv = nameListDiv.cloneNode(true);
        clonedNameListDiv.querySelector('.initials').innerHTML = initials[i];
        mainGameDiv.appendChild(clonedNameListDiv);
    }

    timerEnd = Date.now() + timeLimit;
    updateGameTimer();
};

const updateGameTimer = () => {
    const millisecondsLeft = timerEnd - Date.now();
    const secondsLeft = Math.floor(millisecondsLeft / 1000);
    const minutesLeft = Math.floor(secondsLeft / 60);
    const timerSeconds = secondsLeft % 60;

    if (millisecondsLeft > 0) {
        const gameTimer = document.getElementById('gameTimer');
        gameTimer.innerHTML = `${minutesLeft}:${timerSeconds}`;
        setTimeout(updateGameTimer, 500);
    } else {
        endGame();
        window.alert("Time's up!");
    }
};

const onNameChange = event => {
    const inputElement = event.target;
    const name = inputElement.value;

    if (name.length > 0) {
        const listItem = inputElement.parentElement;
        const checkMark = listItem.querySelector('.validName');
        const errorButton = listItem.querySelector('.invalidName');
        const removeButton = listItem.querySelector('.removeName');

        removeButton.style.display = 'revert';
    
        // if there is no input after this one, add another
        if (listItem.nextElementSibling === null) {
            addNameInput(listItem);
        }

        const listElement = listItem.parentElement;
        const initialsHeader = listElement.previousElementSibling;
        const initials = initialsHeader.innerHTML;
        const [status, validationMessage] = validateName(name, initials);
        errorButton.setAttribute('validationMessage', validationMessage);

        if (status === nameStatus.valid) {
            checkMark.style.display = 'inline flow-root';
            errorButton.style.display = 'none';
        } else if (status === nameStatus.invalid) {
            checkMark.style.display = 'none';
            errorButton.style.display = 'inline flow-root';
        } else {
            checkMark.style.display = 'none';
            errorButton.style.display = 'none';
        }
    }
};

const addNameInput = listItem => {
    const listElement = listItem.parentElement;
    const clonedListItem = listItem.cloneNode(true);
    const clonedInputElement = clonedListItem.querySelector('input');
    const clonedCheckMark = clonedListItem.querySelector('.validName');
    const clonedErrorButton = clonedListItem.querySelector('.invalidName');
    const clonedRemoveButton = clonedListItem.querySelector('.removeName');

    clonedInputElement.value = '';
    clonedCheckMark.style.display = 'none';
    clonedErrorButton.style.display = 'none';
    clonedRemoveButton.style.display = 'none';
    listElement.appendChild(clonedListItem);
};

const nameStatus = {
    valid: 0,
    incomplete: 1,
    invalid: 2
};

const validateName = (name, initials) => {
    const names = name.toUpperCase().split(/\s+/);

    if (names.length > 1) {
        const firstName = names[0];
        const lastName = names[names.length - 1];

        if (firstName[0] === initials[0] && lastName[0] === initials[1]) {
            return [nameStatus.valid, 'OK'];
        } else {
            return [nameStatus.invalid, 'Make sure the first and last name have the correct initials.'];
        }
    }

    return [nameStatus.invalid, 'Make sure to type at least two names, with a space between them'];
};

const onRemoveName = event => {
    const removeButton = event.target;
    const listItem = removeButton.parentElement;
    listItem.remove();
};

const onShowValidationError = event => {
    window.alert(event.target.getAttribute('validationMessage'));
};

const endGame = () => {
    const mainGame = document.getElementById('mainGame');
    mainGame.style.display = 'none';

    beginScoring(getFinalNameList());
};

const getFinalNameList = () => {
    const mainGameDiv = document.getElementById('mainGame');
    const names = [];

    for (const nameListDiv of mainGameDiv.querySelectorAll('.nameList')) {
        console.warn(nameListDiv);
        const nameList = [];

        for (const nameInput of nameListDiv.querySelectorAll('input')) {
            if (nameInput.value.length > 0) {
                console.warn(nameInput.value);
                nameList.push(nameInput.value);
            }
        }

        names.push(nameList);
    }

    return names;
};

/***********
 * Scoring *
 ***********/

const beginScoring = otherNames => {
    console.log(`score other names: ${otherNames}`);

    const scoreNamesDiv = document.getElementById('scoreNames');
    const nameListDiv = scoreNamesDiv.querySelector('.nameList');
    scoreNamesDiv.style.display = 'revert';

    for (let i = 1; i < allInitials.length; i++) {
        const clonedNameListDiv = nameListDiv.cloneNode(true);
        fillNameList(clonedNameListDiv, allInitials[i], otherNames[i]);
        scoreNamesDiv.appendChild(clonedNameListDiv);
    }

    fillNameList(nameListDiv, allInitials[0], otherNames[0]);

    timerEnd = Date.now() + scoreTimeLimit;
    updateScoringTimer();
};

const updateScoringTimer = () => {
    console.log('update scoring timer')
};

const fillNameList = (nameListDiv, initials, names) => {
    if (names.length > 0) {
        nameListDiv.querySelector('.initials').innerHTML = initials;

        const nameListItem = nameListDiv.querySelector('.nameListItem');
        nameListItem.querySelector('.nameDisplay').innerHTML = names[0];
        const nameListElement = nameListItem.parentElement;

        const searchButton = nameListItem.querySelector('.searchName');
        searchButton.setAttribute('href', 'TODO');

        for (let i = 1; i < names.length; i++) {
            const name = names[i];

            const clonedNameListItem = nameListItem.cloneNode(true);
            clonedNameListItem.querySelector('.nameDisplay').innerHTML = name;

            const clonedSearchButton = clonedNameListItem.querySelector('.searchName');
            clonedSearchButton.setAttribute('href', 'TODO');

            nameListElement.appendChild(clonedNameListItem);
        }
    } else {
        nameListDiv.style.display = 'none';
    }
};

const onSubmitRatings = event => {
    console.log('Submitting ratings');
};
