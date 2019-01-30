(define (queens board-size)
  (define (queen-cols k)
    (if (= k 0)
        (list (empty-board board-size))
	(filter
	  (lambda (positions) (safe? k positions))
	  (flatmap
	    (lambda (rest-of-queens)
	      (map (lambda (new-row)
		      (adjoin-position new-row k rest-of-queens board-size))
	           (enumerate-interval 1 board-size)))
	    (queen-cols (- k 1))))))
  (queen-cols board-size))

(define (display-queens board-size)
  (for-each display-position (queens board-size)))

(define (adjoin-position new-row k rest-of-queens board-size)
  (if (= (caar rest-of-queens) 0)
      (list (list 1 (put-at 1 new-row board-size)))
      (append rest-of-queens (list (list k (put-at k new-row board-size))))))

(define (put-at x y size)
  (append (map (lambda (i) (empty-piece x i)) (enumerate-interval 1 (- y 1)))
	  (list (make-piece x y #t))
	  (map (lambda (i) (empty-piece x i)) (enumerate-interval (+ y 1) size))))

(define (safe? k positions)
  (let ((occupied-pieces (filter occupied? (get-col k positions))))
    (if (= (length occupied-pieces) 1)
	(let ((p (car occupied-pieces)))
	  (safe-around? (get-x p) (get-y p) positions))
	#f)))

(define (safe-around? col row positions)
  (and (safe-line? (get-col col positions))
       (safe-line? (get-row row positions))
       (safe-line? (get-diagonal1 col row positions))
       (safe-line? (get-diagonal2 col row positions))))

(define (safe-line? l)
  (= (length (filter occupied? l)) 1))

(define (flatmap f ls)
  (fold-right append () (map f ls)))

(define (enumerate-interval low high)
  (if (> low high)
      ()
      (cons low (enumerate-interval (+ low 1) high))))

;; piece (x y occupied)
(define (empty-board size) 
  (list (list 0 (map (lambda (i) (list 0 i #f)) (enumerate-interval 1 size)))))

(define (make-piece x y o)
  (list x y o))

(define (empty-piece x y)
  (make-piece x y #f))

(define (get-x piece)
  (car piece))

(define (get-y piece)
  (cadr piece))

(define (occupied? piece)
  (caddr piece))

;; positions ((col-index ((x y o) (x y o) ... (x y o) (col-index ((x y o) ...)
;;    col1  col2
;;     ---------->x
;;row1 |
;;row2 |
;;row3 |
;;     \/y
(define (get-col k positions)
  (define (iter p)
    (cond ((null? p) ())
          ((= k (caar p)) (cadar p)) 
	  (else (iter (cdr p)))))
  (iter positions))

(define (get-size positions)
  (length (get-col 1 positions)))

(define (flat-col positions)
  (flatmap cadr positions))

(define (get-row r positions)
  (filter (lambda (p) (= (get-y p) r)) (flat-col positions)))

(define (get-diagonal1 col row positions)
  (define (iter op i result)
    (let ((piece (get-at (op col i) (op row i) positions)))
      (if (null? piece)
	  result
	  (iter op (+ i 1) (append result (list piece))))))
  (append (iter + 0 ()) (iter - 1 ())))

(define (get-diagonal2 col row positions)
  (define (iter op i result)
    (let ((piece (get-at (op col i) (op row (* -1 i)) positions)))
      (if (null? piece)
	  result
	  (iter op (+ i 1) (append result (list piece))))))
  (append (iter + 0 ()) (iter - 1 ())))

(define (get-at x y positions)
  (let ((piece (filter (lambda (p) (= (get-y p) y))
	               (get-col x positions))))
    (if (null? piece)
        ()
	(car piece))))

(define (display-position position)
  (define (iter p)
    (if (< (get-size p) 1)
        (display "\n")
	(begin
          (display-row p)
	  (display "\n")
	  (iter (remove-top-row p)))))
  (iter position))

(define (display-row position)
  (for-each display-piece (map caadr position)))

(define (display-piece piece)
  (if (occupied? piece)
      (display "Q,")
      (display "_,")))

(define (remove-top-row position)
  (map (lambda (row) (list (car row) (cdadr row))) position))

(define unsafe-position1 '((1 ((1 1 #t) (1 2 #f) (1 3 #f)))
	                   (2 ((2 1 #f) (2 2 #f) (2 3 #t)))
	                   (3 ((3 1 #f) (3 2 #t) (3 3 #f)))))

(define queen4 '((1 ((1 1 #f) (1 2 #f) (1 3 #t) (1 4 #f)))
                 (2 ((2 1 #t) (2 2 #f) (2 3 #f) (2 4 #f)))
                 (3 ((3 1 #f) (3 2 #f) (3 3 #f) (3 4 #t)))
                 (4 ((4 1 #f) (4 2 #t) (4 3 #f) (4 4 #f)))))
