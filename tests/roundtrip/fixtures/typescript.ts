import { readFileSync } from "fs";

interface Identifiable {
    id: string;
}

type Result<T, E = Error> = { ok: true; value: T } | { ok: false; error: E };

interface Repository<T extends Identifiable> {
    findById(id: string): T | undefined;
    save(item: T): void;
}

class InMemoryRepo<T extends Identifiable> implements Repository<T> {
    private items: Map<string, T> = new Map();

    findById(id: string): T | undefined {
        return this.items.get(id);
    }

    save(item: T): void {
        this.items.set(item.id, item);
    }
}

function createRepo<T extends Identifiable>(): Repository<T> {
    return new InMemoryRepo<T>();
}

const defaultId: string = "default";

export { InMemoryRepo, createRepo };
