Stock Price Prediction using angular velocity changes in rust
By Joseph Ellsworth 2022-08-22

In my work with stocks estimating prices I have seen that pivotal buying 
or selling opportunities occur after two actions.  For example stock prices 
drop for 3 weeks then stabilize for 1 week.  At these pivotal end points 
prices tend to either resume their longer trend or continue a reversal.
This work is an attempt to develop a classifier which can learn the best
length of time to use when finding similar pairs of movements from the 
past.  

The theory is that certain patterns of these paired movements will result 
in predictable future price movements.  For example if stock prices drop for
3 months and then stabilize or rise slightly for 1 month then prices in the
next 10 days will tend to move upward but if prices drop for 3 months and
stabilize for 1 week they are more likely to resume a downward movement. 
This application seeks to validate this theory by seeking to find a set
of optimal time frames to analyze and then using geometric algebra and
linear regression to determine similarity between past movements.  If the
theory proves valid then we should be able to predict directional movement 
with greater accuracy than the underlying statistical base rate.

This analyzer attempts to predict upward or downward movements by looking
at similar movements in the past.    We use an assertion that longer
trends are better than shorter once provided the deviation from the trend 
is smallest.  To support this assertion the system seeks the longest trend
line between a mix and max of days which yields the smallest error in fit.
It applies this by working backwards from the current data point to find the
length of the short fit line and then starts from the beginning of the
shorter line working backwards to find the longer trend line. The intersection
of those two represents a vertex representation of a velocity change which 
can be measured as a slope of line 2 - slope of line 2 (or we could use trig 
angle from the two slopes). This yields 4 data points:  
   long Line slope,
   long Line length, 
   short Line slope, 
   Short line Length,  
   Angle of velocity change between short line and long line.  

NOTE:  We are using linear regression for by comparing the lines.
  In most uses Linear regression will yield a offset + slope where 
  slope is a absolute value to increase.  Since we want to compare 
  these lines across time when the difference in base price is so 
  large that the absolute change would cause the slopes to be non-comparable
  we convert the slope into a fraction of base price which will allow
  the slopes across different time frames to be compared even if the 
  base price has doubled or tripled.  

To compute the similarity of any single line we can use a formula such as:
    SlopeDelta   = (slope Line - slope Compare Line)
    LengthDelta = (line Length - compare Line Length)
    similarityScore = slopeDelta / LengthDelta

This is intended to allow the slope to be the main driver of similarity but if the lines are radically different lengths reduce the effect similarity.    
When comparing two tuple scores we can use a formula such as
     Sim1 = similarity(mainTuple.line1, compareTuple.line1)
     Sim2 = similarity(maiintuple.line2, compareTuple.line2)
     velChgAngle1 = slopeMainTuple.line2.slope - mainTuple.line1..slope
     velChgAngle2 = compareTubple.line2.slope - compareTuple.line1.slope
     velDelt = 1 / (VelChgAngle1 - velChgAngle2)
     netSim = ( sim1 + sim2)  / velDelt
      
Note: To allow for large price changes we must compute a price movement range from
  min,max prices and compute slope as a fraction of that rather than a pure multiplier. 
  Otherwise for large price movements we could get values for velDelta greater than 0.    
  We encode this data for every bar in the series in a way that allows locating the set 
  of bars with angle that are most similar.  Once we have those bars we can look forward
  a period of time such as 10 bars and look for direction and magnitude of price movement. 
  If we average those price movements together we theoretically can use that data to 
  project a probable price movement for the same time frame from the current bar.  

