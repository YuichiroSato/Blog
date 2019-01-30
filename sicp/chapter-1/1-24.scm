(use srfi-27)

(define (benchmark n times)
  (benchmark-fast n times (runtime)))

(define (runtime)
  (round->exact (* (expt 10 6)
                   (time->seconds (current-time)))))

(define (benchmark-fast n times t)
  (fast-prime? n times)
  (display "sys-time ")
  (display (- (runtime) t))
  (format #t ", prime ~d, times ~d\n" n times))

(define (expmod base exp m)
  (cond ((= exp 0) 1)
        ((even? exp)
         (remainder (square (expmod base (/ exp 2) m))
                    m))
        (else
         (remainder (* base (expmod base (- exp 1) m))
                    m))))        

(define (fermat-test n)
  (define (try-it a)
    (= (expmod a n n) a))
  (try-it (+ 1 (random-integer (- n 1)))))

(define (fast-prime? n times)
  (cond ((= times 0) #t)
        ((fermat-test n) (fast-prime? n (- times 1)))
        (else #f)))
