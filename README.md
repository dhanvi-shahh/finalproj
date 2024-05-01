# finalproj
I used the following resources to iterate through my csv file:
https://doc.rust-lang.org/rust-by-example/flow_control/match.html
https://users.rust-lang.org/t/solved-how-to-fix-this-borrowed-value-does-not-live-long-enough/107105
https://users.rust-lang.org/t/how-to-get-first-column-from-library-rust-csv/4055/2


These resources for helping with Git:
https://stackoverflow.com/questions/4181861/message-src-refspec-master-does-not-match-any-when-pushing-commits-in-git
https://stackoverflow.com/questions/16330404/how-to-remove-remote-origin-from-a-git-repository

These for understanding my dataset and calculating log returns:
https://gregorygundersen.com/blog/2022/02/06/log-returns/ 

I first read the csv file into records. Each record represents a date and the corresponding asset prices on that date. The following is a subset of how this read in looks. 



Following this, I saved the labels, which includes the date and each asset type on that date, as well as the corresponding value of the asset price on that day. The following is a subset:



We grouped together the assets with the most similar log returns. This similarity is based off of a threshold of 99.99. For all assets on every day in our time frame, we assigned edges between the assets that were most similar. Based off of this threshold, we got the following adjacency list:




