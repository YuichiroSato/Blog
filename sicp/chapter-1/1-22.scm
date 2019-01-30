(define (search-for-primes a b)
  (define (iter i n)
    (if (> n 2)
        '()
        (iter (next (find-prime i b)) (+ n 1))))
  (iter a 0))

(define (find-prime a b)
  (define (iter i)
    (cond ((> i b) '())
          ((timed-prime-test i) i)
          (else (iter (next i)))))
  (iter a))

(define (next x)
  (if (even? x)
      (+ x 1)
      (+ x 2)))

(define (timed-prime-test n)
  (start-prime-test n (runtime)))

(define (runtime)
  (round->exact (* (expt 10 6)
                   (time->seconds (current-time)))))

(define (start-prime-test n start-time)
  (if (prime? n)
      (report-prime n (- (runtime) start-time))
       #f))

(define (report-prime n elapsed-time)
  (format #t "prime ~d, sys-time ~d\n" n elapsed-time)
  #t)

(define (smallest-divisor n)
  (find-divisor n 2))

(define (find-divisor n test-divisor)
  (cond ((> (square test-divisor) n) n)
        ((divides? test-divisor n) test-divisor)
        (else (find-divisor n (+ test-divisor 1)))))

(define (divides? a b)
  (= (remainder b a) 0))

(define (square x)
  (* x x))

(define (prime? n)
  (= n (smallest-divisor n)))
