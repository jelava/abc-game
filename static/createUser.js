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

document.addEventListener('DOMContentLoaded', () => {
    const userId = sessionStorage.getItem('userId');
    const oldUserName = localStorage.getItem('userName');

    if (userId) {
        location.pathname='/games.html';
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

const onCreateUser = async event => {
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
    let userId = parseInt(event.data, 10);
    sessionStorage.setItem('userId', userId);
    location.pathname = '/games.html'
};
