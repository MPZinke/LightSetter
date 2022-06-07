
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.31                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


INSERT INTO "Light" ("label", "value") VALUES
('Sink', '3');


INSERT INTO "Event" ("label", "hour", "minute", "value", "Light.id")
SELECT "Temp"."label", "Temp"."hour", "Temp"."minute", "Temp"."value", "Light"."id"
FROM
(
	VALUES
	('Daytime', 8, 0, '"ct": 335' /* white */),
	('Nighttime', 22, 0, '"xy": [0.6867,0.3119]' /* red */)
) AS "Temp"("label", "hour", "minute", "value")
JOIN "Light" ON "Light"."label" = 'Sink';

