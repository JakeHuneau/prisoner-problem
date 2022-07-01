[Prisoner Problem Monte Carlo Simulation]
As seen from [this Veritasium video](https://www.youtube.com/watch?v=iSNsgj1OCLA).
The problem to solve is:

You have `x` prisoners, each numbers from 0...`x`. In another room, there's `x` slips of
papers with a prisoner's number on the back. Each prisoner goes into the room and picks
`y` number of slips. If _every_ prisoner finds their number in `y` guesses, then they are
all free.

The proposed solution in the video is the following: 1. Each prisoner goes into the room and selects the slip with their prisoner number. 2. They check the number for that slip (let that number be `s`). If it's their number, they win. 3. If it's not, then they flip slip `s`. 4. Continue this until they find their number or they run out of guesses

It's also proposed this approaches a win percentage of `1 - ln(2)` which is about 30.7%. This
simulation confirms that.

The file `configuration/base.yaml` is a configuration for this that allows the user to set
the number of prisoners, the number of guesses each prisoner gets, the number of times to
run the simulation, and debug mode. Debug mode will print a check-mark emoji or a cross after
each run if that was a success or failure.
