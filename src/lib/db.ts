
import Database from 'better-sqlite3';
import { join } from 'path';

const dbPath = join(process.cwd(), 'data', 'database.sqlite');
const db = new Database(dbPath);


db.exec(`
    CREATE TABLE IF NOT EXISTS scripts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL, -- Action name
    author TEXT NOT NULL, -- Author of the script
    last_updated TEXT NOT NULL, -- Last updated date
    description TEXT, -- Description of the action
    icon TEXT -- Associated icon
);


INSERT INTO scripts (name, author, last_updated, description, icon) VALUES
('Add Vine Overlay', 'Charlie Simons', '2025-04-27', 'Adds a vine overlay to the interface.', 'vine_icon.png'),
('Apply Weather Effect', 'Alice Johnson', '2023-10-01', 'Applies a weather effect to the background.', 'weather_icon.png'),
('Enable Task Highlights', 'Bob Smith', '2023-09-15', 'Highlights tasks in the task manager.', 'task_icon.png'),
('Play Background Music', 'Charlie Simons', '2023-08-20', 'Plays background music in the app.', 'music_icon.png'),
('Show News Ticker', 'Diana Lee', '2023-07-10', 'Displays a scrolling news ticker.', 'news_icon.png'),
('Track Fitness Goals', 'Evan Brown', '2023-06-05', 'Tracks and displays fitness goals.', 'fitness_icon.png');
`);

export default db;