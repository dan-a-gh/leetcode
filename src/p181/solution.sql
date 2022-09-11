SELECT
	t1.name AS Employee
FROM
	leetcode.Employee t1
JOIN
	leetcode.Employee t2 ON t1.managerId = t2.id
WHERE
	t1.salary > t2.salary