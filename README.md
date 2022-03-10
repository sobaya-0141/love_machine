SQL MEMO
```
DROP TABLE member;
CREATE TABLE member(
  id SERIAL NOT NULL,
  nickname varchar(90) UNIQUE,
  image_url varchar(300),
  PRIMARY KEY (id)
);

DROP TABLE machine;
CREATE TABLE machine(
  id SERIAL NOT NULL,
  name varchar(300),
  image_url varchar(300),
  displacement int,
  machine_type int,
  PRIMARY KEY (id)
);

DROP TABLE machine_type;
CREATE TABLE machine_type(
  id int PRIMARY KEY,
  name varchar(30)
);
INSERT INTO machine_type(id, name) VALUES(0, '車');
INSERT INTO machine_type(id, name) VALUES(1, 'バイク');

DROP TABLE member_machine;
CREATE TABLE member_machine(
  id SERIAL NOT NULL,
  member_id int,
  machine_id int,
  PRIMARY KEY (id)
);
```