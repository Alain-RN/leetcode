class Solution {
    public int lengthOfLongestSubstring(String s) {
        String strSet = "";
        int len = 0;
        for(int i = 0; i < s.length(); i++) {
            if( strSet.indexOf(s.charAt(i)) == -1 ) {

                strSet += s.charAt(i);

                if( len < strSet.length() ) {
                    len = strSet.length();
                }
            } else {
                if( strSet.substring(strSet.indexOf(s.charAt(i)) + 1 ) == "" ) {
                    strSet = "" + s.charAt(i);
                } else {
                    strSet = strSet.substring(strSet.indexOf(s.charAt(i)) + 1 );
                }
            }
            
        }

        return len;
    }
}