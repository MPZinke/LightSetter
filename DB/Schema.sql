
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.30                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


CREATE TABLE "Light"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) UNIQUE NOT NULL DEFAULT '',
	"value" VARCHAR(16) UNIQUE NOT NULL
);


CREATE TABLE "Event"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) UNIQUE NOT NULL DEFAULT '',
	"Light.id" INT NOT NULL,
	FOREIGN KEY ("Light.id") REFERENCES "Light"("id"),
	"hour" INT NOT NULL,
	"minute" INT NOT NULL,
	"value" VARCHAR(128) NOT NULL,
	UNIQUE("label"),
	UNIQUE("Light.id", "hour", "minute")
);
