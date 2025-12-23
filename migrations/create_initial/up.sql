-- Your SQL goes here
CREATE TABLE `messages`(
	`id` BINARY NOT NULL PRIMARY KEY,
	`channel` BINARY NOT NULL,
	`sender` BINARY NOT NULL,
	`time` TIMESTAMP NOT NULL,
	`message` TEXT NOT NULL
);

CREATE TABLE `channels`(
	`id` BINARY NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);