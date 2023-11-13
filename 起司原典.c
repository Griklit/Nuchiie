//输入任意不多于5位的正整数，程序给出这个数的汉字读音
#include <stdio.h>
int main()
{
    int weishu(int x);
    int a,b,w1,w2,w3,w4,w5;
    printf("输入一到五位的整数（按ctrl+Z退出）：");
   //这个代码写得有点失败，因为我在分支时把换行也扔到case里了，我想，如果在读数时不考虑这个，在主函数最后单独输出一个换行，应该就没有这么长了
    while(scanf("%d",&a)!=EOF)
   {
      b=weishu(a);
      if(b==0) printf("零");
      if(b==1)
      {
          switch(a)
          {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
              case 0:printf("零\n");break;
          }
      }
      if(b==2)
      {
          w1=a/10;
          w2=a-10*w1;
          switch(w1)
          {
              case 1:printf("十");break;
              case 2:printf("二十");break;
              case 3:printf("三十");break;
              case 4:printf("四十");break;
              case 5:printf("五十");break;
              case 6:printf("六十");break;
              case 7:printf("七十");break;
              case 8:printf("八十");break;
              case 9:printf("九十");break;
          }
          switch(w2)
          {
              case 0:printf("\n");break;
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
          }
      }
      if(b==3)
      {
          w1=a/100;
          w2=(a-100*w1)/10;
          w3=a-100*w1-10*w2;
          switch(w1)
            {
              case 1:printf("一百");break;
              case 2:printf("二百");break;
              case 3:printf("三百");break;
              case 4:printf("四百");break;
              case 5:printf("五百");break;
              case 6:printf("六百");break;
              case 7:printf("七百");break;
              case 8:printf("八百");break;
              case 9:printf("九百");break;
            }
          if(w3!=0)
          {
             switch(w2)
            {
              case 0:printf("零");break;
              case 1:printf("一十");break;
              case 2:printf("二十");break;
              case 3:printf("三十");break;
              case 4:printf("四十");break;
              case 5:printf("五十");break;
              case 6:printf("六十");break;
              case 7:printf("七十");break;
              case 8:printf("八十");break;
              case 9:printf("九十");break;
            }
            switch(w3)
            {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
            }
          }
          else if(w2==0) printf("\n");
          else switch(w2)
            {
              case 0:break;
              case 1:printf("一十\n");break;
              case 2:printf("二十\n");break;
              case 3:printf("三十\n");break;
              case 4:printf("四十\n");break;
              case 5:printf("五十\n");break;
              case 6:printf("六十\n");break;
              case 7:printf("七十\n");break;
              case 8:printf("八十\n");break;
              case 9:printf("九十\n");break;
            }
      }
      if(b==4)
      {
          w1=a/1000;
          w2=(a-1000*w1)/100;
          w3=(a-1000*w1-100*w2)/10;
          w4=a-1000*w1-100*w2-10*w3;
          switch(w1)
            {
              case 1:printf("一千");break;
              case 2:printf("二千");break;
              case 3:printf("三千");break;
              case 4:printf("四千");break;
              case 5:printf("五千");break;
              case 6:printf("六千");break;
              case 7:printf("七千");break;
              case 8:printf("八千");break;
              case 9:printf("九千");break;
            }
          if(w2!=0)
          {
             switch(w2)
            {
              case 1:printf("一百");break;
              case 2:printf("二百");break;
              case 3:printf("三百");break;
              case 4:printf("四百");break;
              case 5:printf("五百");break;
              case 6:printf("六百");break;
              case 7:printf("七百");break;
              case 8:printf("八百");break;
              case 9:printf("九百");break;
            }
            switch(w3)
            {
              case 0:printf("零");break;
              case 1:printf("一十");break;
              case 2:printf("二十");break;
              case 3:printf("三十");break;
              case 4:printf("四十");break;
              case 5:printf("五十");break;
              case 6:printf("六十");break;
              case 7:printf("七十");break;
              case 8:printf("八十");break;
              case 9:printf("九十");break;
            }
          }
          if(w2==0)
          {
              switch(w3)
            {
              case 0:printf("零");break;
              case 1:printf("零一十");break;
              case 2:printf("零二十");break;
              case 3:printf("零三十");break;
              case 4:printf("零四十");break;
              case 5:printf("零五十");break;
              case 6:printf("零六十");break;
              case 7:printf("零七十");break;
              case 8:printf("零八十");break;
              case 9:printf("零九十");break;
            }
          }
          switch(w4)
            {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
              case 0:printf("\n");break;
             }
      }
      if(b==5)
      {
          w1=a/10000;
          w2=(a-10000*w1)/1000;
          w3=(a-10000*w1-1000*w2)/100;
          w4=(a-10000*w1-1000*w2-100*w3)/10;
          w5=(a-10000*w1-1000*w2-100*w3-10*w4);
          switch(w1)
          {
              case 1:printf("一万");break;
              case 2:printf("二万");break;
              case 3:printf("三万");break;
              case 4:printf("四万");break;
              case 5:printf("五万");break;
              case 6:printf("六万");break;
              case 7:printf("七万");break;
              case 8:printf("八万");break;
              case 9:printf("九万");break;
          }
           /*除万位和个位，5位数中会出现0的情况有：

         ①只出现在十位上

         ②只出现在百位上

         ③只出现在千位上

         ④出现在千、百位上

         ⑤出现在千、十位上

         ⑥出现在百、十位上

         ⑦出现在千、百、十位上*/

          if(w2*w3*w4!=0)//讨论十，百，千位上都不是0的情况
            {
            switch(w2)
            {
              case 1:printf("一千");break;
              case 2:printf("二千");break;
              case 3:printf("三千");break;
              case 4:printf("四千");break;
              case 5:printf("五千");break;
              case 6:printf("六千");break;
              case 7:printf("七千");break;
              case 8:printf("八千");break;
              case 9:printf("九千");break;
            }
            switch(w3)
            {
              case 1:printf("一百");break;
              case 2:printf("二百");break;
              case 3:printf("三百");break;
              case 4:printf("四百");break;
              case 5:printf("五百");break;
              case 6:printf("六百");break;
              case 7:printf("七百");break;
              case 8:printf("八百");break;
              case 9:printf("九百");break;
            }
            switch(w4)
            {
              case 1:printf("一十");break;
              case 2:printf("二十");break;
              case 3:printf("三十");break;
              case 4:printf("四十");break;
              case 5:printf("五十");break;
              case 6:printf("六十");break;
              case 7:printf("七十");break;
              case 8:printf("八十");break;
              case 9:printf("九十");break;
            }
            if(w5==0) printf("\n");
            else switch(w5)
            {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
            }
          }
         if(w2==0&&w3==0&&w4==0&&w5==0) printf("\n");//讨论只有万位上有数字的情况
         if(w2==0&&w3==0&&w4==0&&w5!=0)//情况⑦
         {
             switch(w5)
            {
              case 1:printf("零一\n");break;
              case 2:printf("零二\n");break;
              case 3:printf("零三\n");break;
              case 4:printf("零四\n");break;
              case 5:printf("零五\n");break;
              case 6:printf("零六\n");break;
              case 7:printf("零七\n");break;
              case 8:printf("零八\n");break;
              case 9:printf("零九\n");break;
            }
         }
         if(w2==0&&w3==0&&w4!=0)//情况④
          {
              if(w5==0)//对于每个情况，都要讨论个位是否是0
              {
                  switch(w4)
                 {
                   case 1:printf("零一十\n");break;
                   case 2:printf("零二十\n");break;
                   case 3:printf("零三十\n");break;
                   case 4:printf("零四十\n");break;
                   case 5:printf("零五十\n");break;
                   case 6:printf("零六十\n");break;
                   case 7:printf("零七十\n");break;
                   case 8:printf("零八十\n");break;
                   case 9:printf("零九十\n");break;
                 }
              }
              else
              {
                  switch(w4)
            {
              case 1:printf("零一十");break;
              case 2:printf("零二十");break;
              case 3:printf("零三十");break;
              case 4:printf("零四十");break;
              case 5:printf("零五十");break;
              case 6:printf("零六十");break;
              case 7:printf("零七十");break;
              case 8:printf("零八十");break;
              case 9:printf("零九十");break;
            }
             switch(w5)
            {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
             }
              }
          }
          if(w2!=0&&w3==0&&w4==0)//情况⑥
          {
              switch(w2)
            {
              case 1:printf("一千");break;
              case 2:printf("二千");break;
              case 3:printf("三千");break;
              case 4:printf("四千");break;
              case 5:printf("五千");break;
              case 6:printf("六千");break;
              case 7:printf("七千");break;
              case 8:printf("八千");break;
              case 9:printf("九千");break;
            }
            switch(w5)
            {
              case 0:printf("\n");break;
              case 1:printf("零一\n");break;
              case 2:printf("零二\n");break;
              case 3:printf("零三\n");break;
              case 4:printf("零四\n");break;
              case 5:printf("零五\n");break;
              case 6:printf("零六\n");break;
              case 7:printf("零七\n");break;
              case 8:printf("零八\n");break;
              case 9:printf("零九\n");break;
            }
          }
          if(w2*w3!=0&&w4==0)//情况①
          {
              switch(w2)
            {
              case 1:printf("一千");break;
              case 2:printf("二千");break;
              case 3:printf("三千");break;
              case 4:printf("四千");break;
              case 5:printf("五千");break;
              case 6:printf("六千");break;
              case 7:printf("七千");break;
              case 8:printf("八千");break;
              case 9:printf("九千");break;
            }
             switch(w3)
            {
              case 1:printf("一百");break;
              case 2:printf("二百");break;
              case 3:printf("三百");break;
              case 4:printf("四百");break;
              case 5:printf("五百");break;
              case 6:printf("六百");break;
              case 7:printf("七百");break;
              case 8:printf("八百");break;
              case 9:printf("九百");break;
            }
            if(w5==0) printf("\n");
            else switch(w5)
            {
              case 1:printf("零一\n");break;
              case 2:printf("零二\n");break;
              case 3:printf("零三\n");break;
              case 4:printf("零四\n");break;
              case 5:printf("零五\n");break;
              case 6:printf("零六\n");break;
              case 7:printf("零七\n");break;
              case 8:printf("零八\n");break;
              case 9:printf("零九\n");break;
            }
          }
          if(w2==0&&w3!=0&&w4==0)//情况⑤
          {
              switch(w3)
            {
              case 1:printf("零一百");break;
              case 2:printf("零二百");break;
              case 3:printf("零三百");break;
              case 4:printf("零四百");break;
              case 5:printf("零五百");break;
              case 6:printf("零六百");break;
              case 7:printf("零七百");break;
              case 8:printf("零八百");break;
              case 9:printf("零九百");break;
            }
            if(w5==0) printf("\n");
            else switch(w5)
            {
              case 1:printf("零一\n");break;
              case 2:printf("零二\n");break;
              case 3:printf("零三\n");break;
              case 4:printf("零四\n");break;
              case 5:printf("零五\n");break;
              case 6:printf("零六\n");break;
              case 7:printf("零七\n");break;
              case 8:printf("零八\n");break;
              case 9:printf("零九\n");break;
            }
          }
          if(w2==0&&w3*w4!=0)//情况③
          {
              switch(w3)
            {
              case 1:printf("零一百");break;
              case 2:printf("零二百");break;
              case 3:printf("零三百");break;
              case 4:printf("零四百");break;
              case 5:printf("零五百");break;
              case 6:printf("零六百");break;
              case 7:printf("零七百");break;
              case 8:printf("零八百");break;
              case 9:printf("零九百");break;
            }
            switch(w4)
            {
              case 1:printf("一十");break;
              case 2:printf("二十");break;
              case 3:printf("三十");break;
              case 4:printf("四十");break;
              case 5:printf("五十");break;
              case 6:printf("六十");break;
              case 7:printf("七十");break;
              case 8:printf("八十");break;
              case 9:printf("九十");break;
            }
            if(w5==0) printf("\n");
            else switch(w5)
            {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
            }
          }
          if(w3==0&&w2*w4!=0)//情况②
          {
              switch(w2)
            {
              case 1:printf("一千");break;
              case 2:printf("二千");break;
              case 3:printf("三千");break;
              case 4:printf("四千");break;
              case 5:printf("五千");break;
              case 6:printf("六千");break;
              case 7:printf("七千");break;
              case 8:printf("八千");break;
              case 9:printf("九千");break;
            }
            switch(w4)
            {
              case 1:printf("零一十");break;
              case 2:printf("零二十");break;
              case 3:printf("零三十");break;
              case 4:printf("零四十");break;
              case 5:printf("零五十");break;
              case 6:printf("零六十");break;
              case 7:printf("零七十");break;
              case 8:printf("零八十");break;
              case 9:printf("零九十");break;
            }
            if(w5==0) printf("\n");
            else switch(w5)
            {
              case 1:printf("一\n");break;
              case 2:printf("二\n");break;
              case 3:printf("三\n");break;
              case 4:printf("四\n");break;
              case 5:printf("五\n");break;
              case 6:printf("六\n");break;
              case 7:printf("七\n");break;
              case 8:printf("八\n");break;
              case 9:printf("九\n");break;
            }
          }
      }
  }
  return 0;
}
int weishu(int x)
{
    int y;
    if(x/10000!=0) y=5;
    if(x/10000==0&&x/1000!=0) y=4;
    if(x/10000==0&&x/1000==0&&x/100!=0) y=3;
    if(x/10000==0&&x/1000==0&&x/100==0&&x/10!=0) y=2;
    if(x/10000==0&&x/1000==0&&x/100==0&&x/10==0&&x!=0) y=1;
    if(x==0) y=0;
    return y;
}
