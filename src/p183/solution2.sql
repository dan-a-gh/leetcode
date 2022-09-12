SELECT
	name AS Customers

FROM
	Customers c
	
WHERE 
	c.id NOT IN 
(
	SELECT customerid FROM Orders o
)