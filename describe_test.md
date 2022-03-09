## Manual test

Combination (Linux/Windows/Mac), (user: normal/admin), (befor use inst/befor not use inst), (parameters:8) (parameters with values: 5) - 36 * 8 * 5 (infinity) scenario (without checking parameters) 288 tests * 5 with some data sets of values to check the boundary conditions () assuming that we have 10 such values for one argument, then  288 * 5 * 10 = 14400 scenario ?
1 scenario 1h 7 years tests.

1. Installation on a freshly installed system

- run installer (without parameters) /every comoponent default values internal server database,
- run installer set connection to external server database

2. Instalation on system with have instaled  mysql server before

- run installer (without parameters) /every comoponent default values internal server database,
- run installer set connection to external server database

3. Instalation on system with have instaled java, mysql server before

- run installer (without parameters) /every comoponent default values internal server database,
- run installer set connection to external server database

I Legend

- def internal server database - That is a database installed by the installer in the current directory, initialized and started at scadalts startup on a localhost with root user without password with scadalts database created.
- def external server database - This is an existing mysql database server version 5.7 