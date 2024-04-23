let initial_board = [
    [2, 2, 2, 0, 0, 3],
    [0, 0, 4, 0, 0, 3],
    [1, 1, 4, 0, 0, 3],
    [5, 0, 4, 0, 6, 6],
    [5, 0, 0, 0, 7, 0],
    [8, 8, 8, 0, 7, 0],
 ]
 
 const MAX_ARR_INDEX = 5
 function isGoal(board) {
    //   console.log('isGoal', board)
    return board[2][4] == 1 && board[2][5] == 1
 }
 function prettyPrintBoard(b) {
    let string = ''
    b.forEach((row) => {
       row.forEach((column) => {
          string += `${column}  `
       })
       string += '\n'
    })
    console.log(string)
 }
 console.log('initial_board')
 prettyPrintBoard(initial_board)
 
 function getPossibleNextSteps(board) {
    let car_searched = {} // {id:true}
    let steps = [] // [{carId:left}]
 
    board.forEach((row, x) => {
       row.forEach((carId, y) => {
          if (carId == 0 || car_searched[carId]) return
          car_searched[carId] = true
 
          // Search surrounding for car, from 0,0
          // Search right
          let rightCoordinate = y + 1
          if (
             rightCoordinate <= MAX_ARR_INDEX &&
             board[x][rightCoordinate] == carId
          ) {
             // can car move left?
             let leftCoordindate = y - 1
             if (leftCoordindate >= 0 && board[x][leftCoordindate] == 0) {
                steps.push({ [carId]: 'left' })
             }
 
             // can car move right?
             let moveRightCoordinate = rightCoordinate + 1
             // 3 coordinate car
             if (
                moveRightCoordinate <= MAX_ARR_INDEX &&
                board[x][moveRightCoordinate] == carId
             ) {
                moveRightCoordinate++
             }
 
             if (
                moveRightCoordinate <= MAX_ARR_INDEX &&
                board[x][moveRightCoordinate] == 0
             ) {
                steps.push({ [carId]: 'right' })
             }
          }
 
          // Search down
          let downCoordinate = x + 1
          if (
             downCoordinate <= MAX_ARR_INDEX &&
             board[downCoordinate][y] == carId
          ) {
             // can car move up?
             let upCoordinate = x - 1
             if (upCoordinate >= 0 && board[upCoordinate][y] == 0) {
                steps.push({ [carId]: 'up' })
             }
 
             // can car move down?
             let moveDownCoordinate = downCoordinate + 1
             // 3 coordinate car
             if (
                moveDownCoordinate <= MAX_ARR_INDEX &&
                board[moveDownCoordinate][y] == carId
             ) {
                moveDownCoordinate++
             }
 
             if (
                moveDownCoordinate <= MAX_ARR_INDEX &&
                board[moveDownCoordinate][y] == 0
             ) {
                steps.push({ [carId]: 'down' })
             }
          }
       })
    })
    return steps
 }
 
 function applyStep(board, step) {
    const carId = Object.keys(step)[0]
    const value = Object.values(step)[0]
 
    const newBoard = structuredClone(board)
    if (value == 'up' || value == 'left') {
       loop1: for (let x = 0; x <= MAX_ARR_INDEX; x++) {
          loop2: for (let y = 0; y <= MAX_ARR_INDEX; y++) {
             if (newBoard[x][y] == carId) {
                if (value == 'up') {
                   newBoard[x - 1][y] = Number(carId)
                   if (newBoard[x + 2] && newBoard[x + 2][y] == carId) {
                      newBoard[x + 2][y] = 0
                   } else {
                      newBoard[x + 1][y] = 0
                   }
                   break loop1
                } else if (value == 'left') {
                   newBoard[x][y - 1] = Number(carId)
                   if (newBoard[x][y + 2] && newBoard[x][y + 2] == carId) {
                      newBoard[x][y + 2] = 0
                   } else {
                      newBoard[x][y + 1] = 0
                   }
                   break loop1
                }
             }
          }
       }
    } else {
       loop1: for (let x = 5; x >= 0; x--) {
          loop2: for (let y = 5; y >= 0; y--) {
             if (newBoard[x][y] == carId) {
                if (value == 'down') {
                   newBoard[x + 1][y] = Number(carId)
                   if (newBoard[x - 2] && newBoard[x - 2][y] == carId) {
                      newBoard[x - 2][y] = 0
                   } else {
                      newBoard[x - 1][y] = 0
                   }
                   break loop1
                } else if (value == 'right') {
                   newBoard[x][y + 1] = Number(carId)
                   if (newBoard[x][y - 2] && newBoard[x][y - 2] == carId) {
                      newBoard[x][y - 2] = 0
                   } else {
                      newBoard[x][y - 1] = 0
                   }
                   break loop1
                }
             }
          }
       }
    }
 
    let carId_numberCoord = {}
    newBoard.forEach((row, x) => {
       row.forEach((carId, y) => {
          const carIdTemp = newBoard[x][y]
          if (carIdTemp != 0) {
             if (carId_numberCoord[carIdTemp]) {
                carId_numberCoord[carIdTemp] = carId_numberCoord[carIdTemp] += 1
             } else {
                carId_numberCoord[carIdTemp] = 1
             }
          }
       })
    })
    //    console.log(carId_numberCoord)
    if (Object.values(carId_numberCoord).find((obj) => obj === 1)) {
       console.log('orphan! before')
       //   prettyPrintBoard(board)
       //   console.log(step)
       //   prettyPrintBoard(newBoard)
       throw new Error()
    }
 
    return newBoard
 }
 
 function solve(initial_board) {
    let visited_board = {} // key:board value:any
    let queued_board = [{ [JSON.stringify(initial_board)]: [] }] // [{board:steps}]
    let count = 1
    while (queued_board.length) {
       let keyValue = queued_board.shift()
       const current_board = JSON.parse(Object.keys(keyValue)[0])
       const historical_steps = Object.values(keyValue)[0]
 
       if (isGoal(current_board)) {
          console.log(
             'number of visited_board',
             Object.keys(visited_board).length,
          )
          console.log('goal found', count)
          console.log(historical_steps)
          return
       }
       if (visited_board[JSON.stringify(current_board)]) continue
       visited_board[JSON.stringify(current_board)] = true
 
       let possibleSteps = getPossibleNextSteps(current_board)
       possibleSteps.forEach((step) => {
          const next_board = applyStep(current_board, step)
          if (!visited_board[JSON.stringify(next_board)]) {
             const next_steps = structuredClone(historical_steps)
             next_steps.push(step)
             queued_board.push({ [JSON.stringify(next_board)]: next_steps })
             count++
          }
       })
    }
 }
 solve(initial_board)
 