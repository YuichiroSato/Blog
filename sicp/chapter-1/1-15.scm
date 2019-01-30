(define (cube x) (* x x x))

(define (p x) 
  (- (* 3 x) (* 4 (cube x))))

(define (sine angle)
  (if (not (> (abs angle) 0.1))
      angle
      (p (sine (/ angle 3.0)))))

(define count 0)

(define (sine-count angle)
  (set! count 0)
  (format #t "sin ~d = ~d, p called ~d times\n" angle (iter angle) count))

(define (iter angle)
  (if (not (> (abs angle) 0.1))
      angle
      (p-count (iter (/ angle 3.0)))))

 (define (p-count x)
   (set! count (+ count 1))
   (- (* 3 x) (* 4 (cube x))))

   
