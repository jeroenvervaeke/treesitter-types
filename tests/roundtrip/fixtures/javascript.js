const greeting = "hello";

function add(a, b) {
    return a + b;
}

class Counter {
    constructor(initial) {
        this.count = initial;
    }

    increment() {
        this.count += 1;
    }
}

for (let i = 0; i < 10; i++) {
    console.log(add(i, 1));
}

export { Counter, add };
