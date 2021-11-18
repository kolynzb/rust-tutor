import random
districts = ["kampala","kanungu","nalya","kiwatule","nansana","kisoro","gulu","ntinda","busia","lira"]


def random_word(words=districts,guess="a"):
   word = words[random.randint(0,9)]
   final=""
   for letter in word :
      if(guess == letter):
         final+=guess 
      else:    
        final+="_ "
   if(final.__contains__(guess)) :
        return final
   else:
        return "the letter doesnt exist in the word"


letter = str(input("Guess the letter in the words"))
print(random_word(districts,letter))