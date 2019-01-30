(define (benchmark n)
  (benchmark-fast n (runtime))
  (benchmark-slow n (runtime)))

(define (runtime)
  (round->exact (* (expt 10 6)
                   (time->seconds (current-time)))))

(define (benchmark-fast n t)
  (display (smallest-divisor n))
  (display #\space)
  (display (- (runtime) t))
  (display " fast\n"))

(define (benchmark-slow n t)
  (display (smallest-divisor-slow n))
  (display #\space)
  (display (- (runtime) t))
  (display " slow\n"))

(define (smallest-divisor n)
  (find-divisor n 2))

(define (find-divisor n test-divisor)
  (cond ((> (square test-divisor) n) n)
        ((divides? test-divisor n) test-divisor)
        (else (find-divisor n (next test-divisor)))))

(define (divides? a b)
  (= (remainder b a) 0))

(define (square x)
  (* x x))

(define (next x)
  (if (= x 2)
      3
      (+ x 2)))

(define (smallest-divisor-slow n)
  (find-divisor-slow n 2))

(define (find-divisor-slow n test-divisor)
  (cond ((> (square test-divisor) n) n)
        ((divides? test-divisor n) test-divisor)
        (else (find-divisor-slow n (+ test-divisor 1)))))
