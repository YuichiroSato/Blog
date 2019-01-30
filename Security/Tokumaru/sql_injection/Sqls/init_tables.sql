DROP TABLE Users;
DROP TABLE Books;

CREATE TABLE Users (
  user_id varchar(8) PRIMARY KEY,
  user_name varchar(8),
  password varchar(8)
);

CREATE TABLE Books (
  book_id varchar(8) PRIMARY KEY,
  author varchar(8),
  title varchar(100)
);

GRANT ALL PRIVILEGES ON TABLE Users TO injection_role;
GRANT ALL PRIVILEGES ON TABLE Books TO injection_role;

INSERT INTO Users
VALUES
  ('user1', 'name1', 'passwd1'),
  ('user2', 'name2', 'passwd2')
;

INSERT INTO Books
VALUES
  ('book1', 'author1', 'title1'),
  ('book2', 'author2', 'title2'),
  ('book3', 'author3', 'title3'),
  ('book4', 'author4', 'title4')
;
