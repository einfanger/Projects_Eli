import sqlite3
from datetime import datetime

DB_NAME = "movies.db"


def connect():
    return sqlite3.connect(DB_NAME)


def init_db():
    with connect() as conn:
        cur = conn.cursor()
        cur.execute("""
            CREATE TABLE IF NOT EXISTS movies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                year INTEGER,
                rating REAL CHECK(rating >= 0 AND rating <= 10),
                notes TEXT,
                watched_on TEXT  -- ISO date string: YYYY-MM-DD
            )
        """)
        conn.commit()


def parse_int_or_none(value: str):
    value = value.strip()
    if value == "":
        return None
    try:
        return int(value)
    except ValueError:
        return None


def parse_float_or_none(value: str):
    value = value.strip()
    if value == "":
        return None
    try:
        return float(value)
    except ValueError:
        return None


def parse_date_or_none(value: str):
    value = value.strip()
    if value == "":
        return None
    try:
        datetime.strptime(value, "%Y-%m-%d")
        return value
    except ValueError:
        return None


def print_movie_row(mid, title, year, rating, watched_on, notes=None):
    year_txt = f"({year})" if year else ""
    rating_txt = f"{rating}/10" if rating is not None else "No rating"
    notes_txt = f" | {notes}" if notes else ""
    print(f"[{mid}] {title} {year_txt} - {rating_txt} - watched {watched_on}{notes_txt}")


def add_movie():
    title = input("Title: ").strip()
    if not title:
        print("Title cannot be empty.")
        return

    year_str = input("Year (optional): ")
    year = parse_int_or_none(year_str)
    if year_str.strip() and year is None:
        print("Invalid year. Movie not added.")
        return

    rating_str = input("Rating 0-10 (optional): ")
    rating = parse_float_or_none(rating_str)
    if rating_str.strip() and rating is None:
        print("Invalid rating. Movie not added.")
        return
    if rating is not None and not (0 <= rating <= 10):
        print("Rating must be between 0 and 10. Movie not added.")
        return

    notes = input("Notes (optional): ").strip() or None

    watched_on_str = input("Watched date YYYY-MM-DD (optional, blank = today): ")
    watched_on = parse_date_or_none(watched_on_str)
    if watched_on_str.strip() and watched_on is None:
        print("Invalid date format. Movie not added.")
        return
    if not watched_on:
        watched_on = datetime.now().date().isoformat()

    with connect() as conn:
        cur = conn.cursor()
        cur.execute("""
            INSERT INTO movies (title, year, rating, notes, watched_on)
            VALUES (?, ?, ?, ?, ?)
        """, (title, year, rating, notes, watched_on))
        conn.commit()

    print("Movie added.")


def list_movies():
    with connect() as conn:
        cur = conn.cursor()
        cur.execute("""
            SELECT id, title, year, rating, watched_on, notes
            FROM movies
            ORDER BY watched_on DESC, title ASC
        """)
        rows = cur.fetchall()

    if not rows:
        print("No movies found.")
        return

    print("\n--- Your Movies ---")
    for row in rows:
        print_movie_row(*row)
    print()


def search_movies():
    term = input("Search title contains: ").strip()
    if term == "":
        print("Search term cannot be empty.")
        return

    with connect() as conn:
        cur = conn.cursor()
        cur.execute("""
            SELECT id, title, year, rating, watched_on, notes
            FROM movies
            WHERE title LIKE ?
            ORDER BY watched_on DESC, title ASC
        """, (f"%{term}%",))
        rows = cur.fetchall()

    if not rows:
        print("No matches found.")
        return

    print("\n--- Matches ---")
    for row in rows:
        print_movie_row(*row)
    print()


def update_movie():
    try:
        movie_id = int(input("Movie ID to update: ").strip())
    except ValueError:
        print("Invalid ID.")
        return

    with connect() as conn:
        cur = conn.cursor()
        cur.execute("""
            SELECT id, title, year, rating, watched_on, notes
            FROM movies
            WHERE id = ?
        """, (movie_id,))
        existing = cur.fetchone()

    if not existing:
        print("No movie found with that ID.")
        return

    print("\nCurrent values:")
    print_movie_row(*existing)

    print("\nLeave blank to keep existing value.")

    rating_str = input("New rating 0-10: ")
    new_rating = parse_float_or_none(rating_str)
    if rating_str.strip() and new_rating is None:
        print("Invalid rating.")
        return
    if new_rating is not None and not (0 <= new_rating <= 10):
        print("Rating must be between 0 and 10.")
        return

    new_notes = input("New notes: ").strip()

    watched_on_str = input("New watched date YYYY-MM-DD: ")
    new_watched_on = parse_date_or_none(watched_on_str)
    if watched_on_str.strip() and new_watched_on is None:
        print("Invalid date format.")
        return

    updates = []
    values = []

    if new_rating is not None:
        updates.append("rating = ?")
        values.append(new_rating)

    if new_notes != "":
        updates.append("notes = ?")
        values.append(new_notes)

    if new_watched_on:
        updates.append("watched_on = ?")
        values.append(new_watched_on)

    if not updates:
        print("Nothing to update.")
        return

    values.append(movie_id)

    with connect() as conn:
        cur = conn.cursor()
        cur.execute(f"""
            UPDATE movies
            SET {", ".join(updates)}
            WHERE id = ?
        """, values)
        conn.commit()

        if cur.rowcount == 0:
            print("No movie found with that ID.")
        else:
            print("Movie updated.")


def delete_movie():
    try:
        movie_id = int(input("Movie ID to delete: ").strip())
    except ValueError:
        print("Invalid ID.")
        return

    with connect() as conn:
        cur = conn.cursor()
        cur.execute("SELECT id, title, watched_on FROM movies WHERE id = ?", (movie_id,))
        row = cur.fetchone()

    if not row:
        print("No movie found with that ID.")
        return

    mid, title, watched_on = row
    print(f"Selected: [{mid}] {title} - watched {watched_on}")

    confirm = input("Type DELETE to confirm: ").strip()
    if confirm != "DELETE":
        print("Deletion cancelled.")
        return

    with connect() as conn:
        cur = conn.cursor()
        cur.execute("DELETE FROM movies WHERE id = ?", (movie_id,))
        conn.commit()

        if cur.rowcount == 0:
            print("No movie found with that ID.")
        else:
            print("Movie deleted.")


def list_by_date_range():
    start = input("Start date YYYY-MM-DD: ").strip()
    end = input("End date YYYY-MM-DD: ").strip()

    if not parse_date_or_none(start) or not parse_date_or_none(end):
        print("Invalid date format.")
        return

    with connect() as conn:
        cur = conn.cursor()
        cur.execute("""
            SELECT id, title, year, rating, watched_on, notes
            FROM movies
            WHERE watched_on BETWEEN ? AND ?
            ORDER BY watched_on ASC, title ASC
        """, (start, end))
        rows = cur.fetchall()

    if not rows:
        print("No movies found in that date range.")
        return

    print("\n--- Movies in Date Range ---")
    for row in rows:
        print_movie_row(*row)
    print()


def show_stats():
    with connect() as conn:
        cur = conn.cursor()
        cur.execute("SELECT COUNT(*) FROM movies;")
        total_movies = cur.fetchone()[0]

        cur.execute("""
            SELECT COUNT(rating), AVG(rating)
            FROM movies
            WHERE rating IS NOT NULL
        """)
        rated_count, avg_rating = cur.fetchone()

        cur.execute("""
            SELECT title, year, rating, watched_on
            FROM movies
            WHERE rating IS NOT NULL
            ORDER BY rating DESC, watched_on DESC
            LIMIT 1
        """)
        top = cur.fetchone()

    print("\n--- Statistics ---")
    print(f"Total movies in database: {total_movies}")
    print(f"Movies with ratings: {rated_count}")
    if avg_rating is None:
        print("Average rating: N/A")
    else:
        print(f"Average rating: {round(avg_rating, 2)}/10")

    if top:
        title, year, rating, watched_on = top
        year_txt = f" ({year})" if year else ""
        print(f"Top rated movie: {title}{year_txt} - {rating}/10 (watched {watched_on})")
    else:
        print("Top rated movie: N/A")

    print()


def menu():
    print("""
====================================
        Movie Rating Database
====================================
1) Add a movie
2) List all movies
3) Search movies by title
4) Update a movie
5) Delete a movie
6) List movies by date range
7) Show statistics
0) Exit
""")


def main():
    init_db()

    while True:
        menu()
        choice = input("Choose: ").strip()

        if choice == "1":
            add_movie()
        elif choice == "2":
            list_movies()
        elif choice == "3":
            search_movies()
        elif choice == "4":
            update_movie()
        elif choice == "5":
            delete_movie()
        elif choice == "6":
            list_by_date_range()
        elif choice == "7":
            show_stats()
        elif choice == "0":
            print("Goodbye.")
            break
        else:
            print("Invalid choice.")


if __name__ == "__main__":
    main()