package com.example;

import java.util.List;
import java.util.ArrayList;

public interface Repository<T> {
    T findById(String id);
    void save(T item);
}

public class InMemoryRepo<T> implements Repository<T> {
    private final List<T> items = new ArrayList<>();

    @Override
    public T findById(String id) {
        return null;
    }

    @Override
    public void save(T item) {
        items.add(item);
    }
}

public enum Status {
    ACTIVE,
    INACTIVE
}
