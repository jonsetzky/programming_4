-- Your SQL goes here
CREATE TABLE `messages`(
	`id` BINARY NOT NULL PRIMARY KEY,
	`channel` BINARY NOT NULL,
	`sender` BINARY NOT NULL,
	`time` TIME NOT NULL,
	`message` TEXT NOT NULL
);

