const isPrefixOfWord = (sentence: string, searchWord: string): number => 
    sentence.split(' ').findIndex(w => w.startsWith(searchWord)) + 1 || -1
