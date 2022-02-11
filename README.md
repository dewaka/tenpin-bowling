# Ten Pin Bowling

* Game consists of 10 frames
* In each frame a player has 2 opportunities to knock down 10 pins
* The score for the frame is the total number of pins knocked down, plus bonuses for strikes and spares
  * A spare is when a player knocks down all 10 pins in two tries. The bonus for that frame is the number of pins knocked down by the next roll [E.g, if a bowler rolls, 4,6 | 5,0, then their score is 20 = (4 + 6 + 5) + (5 + 0)]
  * A strike is when a player knocks down all 10 pins on their first try. The value for that frame is the value of the next two balls. [E.g, if a bowler rolls, 10 | 5, 4, then their score is 28 = (10 + 5 + 4) + (5 + 4)]
 * A spare or strike in the 10th frame allows the player to roll extra balls to complete the frame, however no more than
 3 balls can be rolled in the 10th.
