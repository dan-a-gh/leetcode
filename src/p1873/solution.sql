SELECT
	employee_id,
	IF (employee_id % 2 AND name NOT LIKE "M%", salary, 0) AS bonus
	
FROM
	Employees e
	
ORDER BY
	employee_id
	