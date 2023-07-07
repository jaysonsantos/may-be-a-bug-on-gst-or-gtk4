Before the pipeline starts, the overlayed widget keeps the right size.
<img src="img-01.png">

Also the GtkPicture which would have the video painted onto.
<img src="img-02.png">

But, as soon as the pipeline starts, the upper widget gets resized.
<img src="img-03.png">
Even though the GtkPicture being scaled down propertly, it seems that the parent expands without reason.
<img src="img-04.png">