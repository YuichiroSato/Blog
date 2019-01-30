(define (run-test)
  (test 561)
  (test 1105)
  (test 1729)
  (test 2465)
  (test 2821)
  (test 6601))

(define (test a)
  (format #t "~a is prime? " a)
  (display (test-all-a a))
  (display "\n"))

(define (expmod base exp m)
  (cond ((= exp 0) 1)
        ((even? exp)
         (remainder (square (expmod base (/ exp 2) m))
                    m))
        (else
         (remainder (* base (expmod base (- exp 1) m))
                    m))))        

(define (fermat-test n a)
  (= (expmod a n n) a))

(define (fast-prime? n a times)
  (cond ((= times 0) #t)
        ((fermat-test n a) (fast-prime? n a (- times 1)))
        (else #f)))

(define (test-all-a n)
  (define (test-all-a-iter i result)
    (if (>= i n)
        result
        (test-all-a-iter (+ i 1) (and result (fast-prime? n i 10)))))
  (test-all-a-iter 1 #t))
