(use srfi-27)

(define (run-test)
  (test-under 50)
  (test 561)
  (test 1105)
  (test 1729)
  (test 2465)
  (test 2821)
  (test 6601))

(define (test n)
  (format #t "~d is prime? " n)
  (display (fast-prime? n 10))
  (display "\n"))

(define (test-under n)
  (define (iter i udf)
    (if (> i n)
        (display "\n")
        (iter (+ i 2) (test i))))
  (iter 5 #f))

(define (expmod base exp m)
  (cond ((square-check base m) 0)
        ((= exp 0) 1)
        ((even? exp)
         (remainder (square (expmod base (/ exp 2) m))
                    m))
        (else
         (remainder (* base (expmod base (- exp 1) m))
                    m))))        

(define (square-check x m)
  (and (not (= x 1))
       (not (= x (- m 1)))
       (= (remainder (square x) m) 1)))

(define (miller-rabin-test n a)
  (better-check (expmod a (- n 1) n)))

(define (better-check x)
  (and (= x 1) (not (= x 0))))

(define (fast-prime? n times)
  (cond ((= times 0) #t)
        ((miller-rabin-test n (+ 1 (random-integer (- n 1))))  (fast-prime? n (- times 1)))
        (else #f)))
