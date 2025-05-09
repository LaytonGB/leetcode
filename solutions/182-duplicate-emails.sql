# Write your MySQL query statement below
SELECT DISTINCT P1.email as Email
FROM Person as P1, Person as P2
WHERE P1.id != P2.id AND P1.email = P2.email