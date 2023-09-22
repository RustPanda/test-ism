## Задача
 
Необходимо разработать CLI-утилиту совершающую HTTP Healthcheck'и по заданному URL'у.
 
Проверка совершается в цикле с заданным интервалом. На каждой итерации необходимо совершить HTTP GET по заданному URL'у.
Есть три возможных результата проверки:
1. `OK(200)`, в случае, если запрос вернул HTTP status code `200`.
2. `ERR({HTTP_CODE})`, в случае, если запрос вернул HTTP status code отличный от `200`.
3. `URL parsing error`, в случае, если второй аргумент не является валидным HTTP URL'ом. После чего приложение завершается.
 
Утилита принимает два аргумента:
1. Целочисленное значение интервала в секундах.
2. HTTP URL который будет проверяться.
 
Примеры выполнения:
```
~$./my_binary 2 http://example.com/
 
Checking 'http://example.com/'. Result: OK(200)
Checking 'http://example.com/'. Result: OK(200)
Checking 'http://example.com/'. Result: OK(200)
^C
```
```
~$./my_binary 1 http://httpstat.us/500
 
Checking 'http://httpstat.us/500'. Result: ERR(500)
Checking 'http://httpstat.us/500'. Result: ERR(500)
^C
```
```
~$./my_binary 1 http://httpstat.us/503
 
Checking 'http://httpstat.us/503'. Result: ERR(503)
Checking 'http://httpstat.us/503'. Result: ERR(503)
^C
```
```
~$./my_binary 1 this_is_not_an_url
 
URL parsing error
```