const letters = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

const numInitials = 26;
let allInitials = [];

const timeLimit = 1000 * 60 * 0.5;
let timerEnd = Date.now();

/******************
 * Hosting a game *
 ******************/

const onHost = event => {
    document.getElementById('hostOrConnect').style.display = 'none';
    document.getElementById('hostGame').style.display = 'revert';
};

const onHostCancel = event => {
    console.log('host canceled');
    document.getElementById('hostGame').style.display = 'none';
    document.getElementById('hostOrConnect').style.display = 'revert';
};

const onHostStart = event => {
    pickInitials();
    document.getElementById('hostGame').style.display = 'none';
    startGame();
};

const pickInitials = () => {
    let r1 = 0;
    let r2 = 0;
    let initials = '';

    while (allInitials.length < numInitials) {
        r1 = Math.floor(letters.length * Math.random());
        r2 = Math.floor(letters.length * Math.random());
        initials = letters[r1] + letters[r2];

        if (allInitials.indexOf(initials) < 0) {
            allInitials.push(initials);
        }
    }
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

 const startGame = () => {
    document.getElementById('mainGame').style.display = 'revert';

    // create N-1 copies of the nameList div and sets initials for each one
    const mainGameDiv = document.getElementById('mainGame')
    const nameListDiv = mainGameDiv.querySelector('.nameList');
    nameListDiv.querySelector('.initials').innerHTML = allInitials[0];

    for (let i = 1; i < allInitials.length; i++) {
        const clonedNameListDiv = nameListDiv.cloneNode(true);
        clonedNameListDiv.querySelector('.initials').innerHTML = allInitials[i];
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
