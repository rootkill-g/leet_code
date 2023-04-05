# Understanding the problem statement:

- If the number of rows is 1, the string is returned as is, because it can only be one row.

- In other cases (let's suppose 4 rows):

Let's suppose the string to be -> `INTERNATIONALIZATION`

So in the matrix with 4 rows and n columns (we count from 0, 3 for 4 rows)
```
  I will take (0, 0)th position
  N will take (1, 0)th position
  T will take (2, 0)th position
  E will take (3, 0)th position
  R will take (2, 1)th position 
  N will take (1, 2)th position
  A will take (0, 3)th position
  T will take (1, 3)th position
  I will take (2, 3)th position
  O will take (3, 3)th position
  N will take (2, 4)th position
  A will take (1, 5)th position
  L will take (0, 6)th position
  I will take (1, 6)th position
  Z will take (2, 6)th position
  A will take (3, 6)th position
  T will take (2, 7)th position
  I will take (1, 8)th position
  O will take (0, 9)th position
  N will take (1, 9)th position
```

- The pattern will look like this:
```
I  A  L  O
N NT AI IN
TR IN ZT
E  O  A
```

We now take characters from 0th row 0th column and travel through the 0th column
and add all the occuring characters in order, when there are now more characters
in the row we start taking characters from the next row.

- So the output string will be : `IALONNTAIINTRINZTEOA`
