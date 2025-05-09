# Write your MySQL query statement below
SELECT W2.id FROM Weather as W1, Weather as W2
WHERE W2.recordDate = DATE_ADD(W1.recordDate, INTERVAL 1 DAY) AND W2.temperature > W1.temperature