# FIDL compiler error catalog

This document lists all errors emitted by the [FIDL compiler][docs-fidlc],
`fidlc`. Error identifiers in this domain are always rendered with the prefix
`fi-` followed by a four digit code, like `fi-0123`.

<!--

// LINT.IfChange

-->

<<error-catalog/_fi-0001.md>>

<<error-catalog/_fi-0002.md>>

<<error-catalog/_fi-0003.md>>

<<error-catalog/_fi-0004.md>>

<<error-catalog/_fi-0005.md>>

<<error-catalog/_fi-0006.md>>

<<error-catalog/_fi-0007.md>>

<<error-catalog/_fi-0008.md>>

<<error-catalog/_fi-0009.md>>

<<error-catalog/_fi-0010.md>>

<<error-catalog/_fi-0011.md>>

<<error-catalog/_fi-0012.md>>

<<error-catalog/_fi-0013.md>>

<<error-catalog/_fi-0014.md>>

<<error-catalog/_fi-0015.md>>

<<error-catalog/_fi-0016.md>>

<<error-catalog/_fi-0017.md>>

<<error-catalog/_fi-0018.md>>

<<error-catalog/_fi-0019.md>>

<<error-catalog/_fi-0020.md>>

<<error-catalog/_fi-0021.md>>

<<error-catalog/_fi-0022.md>>

<<error-catalog/_fi-0023.md>>

<<error-catalog/_fi-0024.md>>

<<error-catalog/_fi-0025.md>>

<<error-catalog/_fi-0026.md>>

<<error-catalog/_fi-0027.md>>

<<error-catalog/_fi-0028.md>>

<<error-catalog/_fi-0029.md>>

<<error-catalog/_fi-0030.md>>

<<error-catalog/_fi-0031.md>>

<<error-catalog/_fi-0032.md>>

<<error-catalog/_fi-0033.md>>

<<error-catalog/_fi-0034.md>>

<<error-catalog/_fi-0035.md>>

<<error-catalog/_fi-0036.md>>

<<error-catalog/_fi-0037.md>>

<<error-catalog/_fi-0038.md>>

<<error-catalog/_fi-0039.md>>

<<error-catalog/_fi-0040.md>>

<<error-catalog/_fi-0041.md>>

<<error-catalog/_fi-0042.md>>

<<error-catalog/_fi-0043.md>>

<<error-catalog/_fi-0044.md>>

<<error-catalog/_fi-0045.md>>

<<error-catalog/_fi-0046.md>>

<<error-catalog/_fi-0047.md>>

<<error-catalog/_fi-0048.md>>

<<error-catalog/_fi-0049.md>>

<<error-catalog/_fi-0050.md>>

<<error-catalog/_fi-0051.md>>

<<error-catalog/_fi-0052.md>>

<<error-catalog/_fi-0053.md>>

<<error-catalog/_fi-0054.md>>

<<error-catalog/_fi-0055.md>>

<<error-catalog/_fi-0056.md>>

<<error-catalog/_fi-0057.md>>

<<error-catalog/_fi-0058.md>>

<<error-catalog/_fi-0059.md>>

<<error-catalog/_fi-0060.md>>

<<error-catalog/_fi-0061.md>>

<<error-catalog/_fi-0062.md>>

<<error-catalog/_fi-0063.md>>

<<error-catalog/_fi-0064.md>>

<<error-catalog/_fi-0065.md>>

<<error-catalog/_fi-0066.md>>

<<error-catalog/_fi-0067.md>>

<<error-catalog/_fi-0068.md>>

<<error-catalog/_fi-0069.md>>

<<error-catalog/_fi-0070.md>>

<<error-catalog/_fi-0071.md>>

<<error-catalog/_fi-0072.md>>

<<error-catalog/_fi-0073.md>>

<<error-catalog/_fi-0074.md>>

<<error-catalog/_fi-0075.md>>

<<error-catalog/_fi-0076.md>>

<<error-catalog/_fi-0077.md>>

<<error-catalog/_fi-0078.md>>

<<error-catalog/_fi-0079.md>>

<<error-catalog/_fi-0080.md>>

<<error-catalog/_fi-0081.md>>

<<error-catalog/_fi-0082.md>>

<<error-catalog/_fi-0083.md>>

<<error-catalog/_fi-0084.md>>

<<error-catalog/_fi-0085.md>>

<<error-catalog/_fi-0086.md>>

<<error-catalog/_fi-0087.md>>

<<error-catalog/_fi-0088.md>>

<<error-catalog/_fi-0089.md>>

<<error-catalog/_fi-0090.md>>

<<error-catalog/_fi-0091.md>>

<<error-catalog/_fi-0092.md>>

<<error-catalog/_fi-0093.md>>

<<error-catalog/_fi-0094.md>>

<<error-catalog/_fi-0095.md>>

<<error-catalog/_fi-0096.md>>

<<error-catalog/_fi-0097.md>>

<<error-catalog/_fi-0098.md>>

<<error-catalog/_fi-0099.md>>

<<error-catalog/_fi-0100.md>>

<<error-catalog/_fi-0101.md>>

<<error-catalog/_fi-0102.md>>

<<error-catalog/_fi-0103.md>>

<<error-catalog/_fi-0104.md>>

<<error-catalog/_fi-0105.md>>

<<error-catalog/_fi-0106.md>>

<<error-catalog/_fi-0107.md>>

<<error-catalog/_fi-0108.md>>

<<error-catalog/_fi-0109.md>>

<<error-catalog/_fi-0110.md>>

<<error-catalog/_fi-0111.md>>

<<error-catalog/_fi-0112.md>>

<<error-catalog/_fi-0113.md>>

<<error-catalog/_fi-0114.md>>

<<error-catalog/_fi-0115.md>>

<<error-catalog/_fi-0116.md>>

<<error-catalog/_fi-0117.md>>

<<error-catalog/_fi-0118.md>>

<<error-catalog/_fi-0119.md>>

<<error-catalog/_fi-0120.md>>

<<error-catalog/_fi-0121.md>>

<<error-catalog/_fi-0122.md>>

<<error-catalog/_fi-0123.md>>

<<error-catalog/_fi-0124.md>>

<<error-catalog/_fi-0125.md>>

<<error-catalog/_fi-0126.md>>

<<error-catalog/_fi-0127.md>>

<<error-catalog/_fi-0128.md>>

<<error-catalog/_fi-0129.md>>

<<error-catalog/_fi-0130.md>>

<<error-catalog/_fi-0131.md>>

<<error-catalog/_fi-0132.md>>

<<error-catalog/_fi-0133.md>>

<<error-catalog/_fi-0134.md>>

<<error-catalog/_fi-0135.md>>

<<error-catalog/_fi-0136.md>>

<<error-catalog/_fi-0137.md>>

<<error-catalog/_fi-0138.md>>

<<error-catalog/_fi-0139.md>>

<<error-catalog/_fi-0140.md>>

<<error-catalog/_fi-0141.md>>

<<error-catalog/_fi-0142.md>>

<<error-catalog/_fi-0143.md>>

<<error-catalog/_fi-0144.md>>

<<error-catalog/_fi-0145.md>>

<<error-catalog/_fi-0146.md>>

<<error-catalog/_fi-0147.md>>

<<error-catalog/_fi-0148.md>>

<<error-catalog/_fi-0149.md>>

<<error-catalog/_fi-0150.md>>

<<error-catalog/_fi-0151.md>>

<<error-catalog/_fi-0152.md>>

<<error-catalog/_fi-0153.md>>

<<error-catalog/_fi-0154.md>>

<<error-catalog/_fi-0155.md>>

<<error-catalog/_fi-0156.md>>

<<error-catalog/_fi-0157.md>>

<<error-catalog/_fi-0158.md>>

<<error-catalog/_fi-0159.md>>

<<error-catalog/_fi-0160.md>>

<<error-catalog/_fi-0161.md>>

<<error-catalog/_fi-0162.md>>

<<error-catalog/_fi-0163.md>>

<<error-catalog/_fi-0164.md>>

<<error-catalog/_fi-0165.md>>

<<error-catalog/_fi-0166.md>>

<<error-catalog/_fi-0167.md>>

<<error-catalog/_fi-0168.md>>

<<error-catalog/_fi-0169.md>>

<<error-catalog/_fi-0170.md>>

<<error-catalog/_fi-0171.md>>

<<error-catalog/_fi-0172.md>>

<<error-catalog/_fi-0173.md>>

<<error-catalog/_fi-0174.md>>

<<error-catalog/_fi-0175.md>>

<<error-catalog/_fi-0176.md>>

<<error-catalog/_fi-0177.md>>

<<error-catalog/_fi-0178.md>>

<<error-catalog/_fi-0179.md>>

<<error-catalog/_fi-0180.md>>

<<error-catalog/_fi-0181.md>>

<<error-catalog/_fi-0182.md>>

<<error-catalog/_fi-0183.md>>

<<error-catalog/_fi-0184.md>>

<<error-catalog/_fi-0185.md>>

<<error-catalog/_fi-0186.md>>

<<error-catalog/_fi-0187.md>>

<<error-catalog/_fi-0188.md>>

<<error-catalog/_fi-0189.md>>

<<error-catalog/_fi-0190.md>>

<<error-catalog/_fi-0191.md>>

<<error-catalog/_fi-0192.md>>

<<error-catalog/_fi-0193.md>>

<<error-catalog/_fi-0194.md>>

<<error-catalog/_fi-0195.md>>

<<error-catalog/_fi-0196.md>>

<!--

// LINT.ThenChange(/tools/fidl/fidlc/include/fidl/diagnostics.h)

-->

[docs-fidlc]: ../language/fidlc.md




