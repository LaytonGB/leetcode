# Write your MySQL query statement below
select p.firstName firstName, p.lastName lastName, a.city city, a.state state
from Person as p
left join Address as a on p.personId = a.personId