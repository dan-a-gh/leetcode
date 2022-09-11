SELECT
	email AS Email

FROM
	leetcode.Person
	
GROUP BY
	email

HAVING
	COUNT(*) > 1
