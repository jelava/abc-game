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
    'lengthy',
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

const onCreateUser = async event => {
    const userName = document.getElementById('userNameInput').value;

    if (userName.length >= 3) {
        // TODO: error handling

        let userId = await fetch(`/users/${userName}`, { method: 'POST' })
            .then(response => response.json())
            .then(json => json.user_id);

        console.log(`user ID: ${userId}`);
        sessionStorage.setItem('userId', userId);
        location.pathname='/index.html'
    } else {
        alert('Please choose a nickname with at least three letters.');
    }
};

const onRandomizeName = () => {
    const r1 = Math.floor(Math.random() * adjectives.length);
    const r2 = Math.floor(Math.random() * animals.length);
    document.getElementById('userNameInput').value = `${adjectives[r1]} ${animals[r2]}`;
};

document.addEventListener('DOMContentLoaded', onRandomizeName);
