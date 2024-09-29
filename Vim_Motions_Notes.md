Vim motions Notes
=================

Go To
-----
| Go To | Command |
| ----------- | ------------- |
| basic movement | hjkl |
| start of line (col 0) | 0 |
| start of line (first non blank char) | _ or ^ |
| end of line | $ |
| end/start of code block | % |


Search
------
| Action | Command | Example |
| ------ | ------- | --------|
| Search next char | f[char] | f( goes to next ( |
| Search and highlight | vi[char] | vi( search next ( and highligths the group |

- The i in compound commands represents inner, like inner block. For example vi( highlights next inner group after ( char

Switch to INSERT Mode
---------------------
| Action | Command |
| -------| ------- |
| Append at end of line | A |
| Append at current pos | a |
| Insert at the start of the line | I |
| Insert at current pos | i |




Basic Cursor Movement keys hjkl
    Go to start of line: 0 (col 0)
    Go to start of line text: _ (col X where X is the first non empty char)
