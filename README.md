# ICRA

International Cultivar Registration Authorities (ICRAs) are organisations responsible for registering/recording plant cultivars.

This library aims to provide a common interface to as many of the ICRA databases as possible.

https://www.ishs.org/sci/icralist/icralist.htm

# Status

* ~~AFRICAN VIOLET SOCIETY OF AMERICA, INC. (A.V.S.A.)~~
  * List isn't available online
* ~~AGRICULTURAL UNIVERSITY OF HEBEI~~
  * Entry just has an email address
* AMERICAN BEGONIA SOCIETY
  * One big HTML file with some fancy client-side JavaScript pagination
  * Initial work started
* AMERICAN BOXWOOD SOCIETY (A.B.S.)
  * Downloadable PDF (https://www.boxwoodsociety.org/art/abs_downloads/ABS_Checklist_v2_FINAL_sm.pdf)
* AMERICAN FUCHSIA SOCIETY
  * Embedded Google Sheet
* AMERICAN HEMEROCALLIS SOCIETY
  * URLS with an ID at the end (https://daylilydatabase.org/detail.php?id=145055 is earliest I can find)
* AMERICAN HOSTA SOCIETY
  * URLS with an ID at the end (https://www.hostaregistrar.org/detail.php?id=50012)
  * Has a "Show all" button (https://www.hostaregistrar.org/search.php)
* ~~AMERICAN IRIS SOCIETY~~
  * Requires membership to access the list
* AMERICAN IVY SOCIETY (A.I.S.)
  * HTML pages (http://www.ivy.org/gallery1.htm, http://www.ivy.org/gallery2.htm, http://www.ivy.org/gallery3.htm)
* ~~AMERICAN PENSTEMON SOCIETY~~
  * Entry just has an email
* AMERICAN PEONY SOCIETY
  * JSON endpoint (https://americanpeonysociety.org/cultivars/peony-registry/?registry_data=true)
* ~~AMERICAN PRIMROSE SOCIETY (APS)~~
  * No information on their website
* AMERICAN PUBLIC GARDENS ASSOCIATION
  * Multiple genera
  * Apparently appears in [HortScience](https://journals.ashs.org/hortsci/view/journals/hortsci/hortsci-overview.xml)
* AMERICAN ROSE SOCIETY (A.R.S.)
  * Database available online (https://modernroses.org/details.php?cultivar=37133)
  * IDs seem to start at 1
* ARBORETUM KALMTHOUT FOUNDATION
  * Embedded slides from https://www.arboretumkalmthout.be/ontdek/hamamelis/cultivar-registratie.html
* AUSTRALIAN CULTIVAR REGISTRATION AUTHORITY (ACRA)
  * Multiple genera
  * List is located at https://acra.biodiversity.services/info/list
* CHINA NATIONAL BOTANICAL GARDEN
  * Main search page (keep blank fields to view all) https://www.malusregister.org/en/search
  * UUIDs: https://www.malusregister.org/en/sjk_nr?id=4028b881820797ce018207ea8ba10e1a
* ~~BLAHNIK, ING. ZDENEK~~
  * Entry just has an email
* BROMELIAD SOCIETY INTERNATIONAL (B.S.I.)
  * Multiple genera in the Bromeliad family
  * https://bsi.org/registry/?genus=ANANAS&id=644
* BRUGMANSIA GROWERS INTERNATIONAL
  * Brugmansia and Datura
  * https://www.brugmansia.us/hybridizer-database/
  * https://www.brugmansia.us/datura-cultivar-register/
* ~~MEI FLOWER AND WINTERSWEET BRANCH OF CHINA FLOWER ASSOCIATION - CHIMONANTHUS~~
  * Entry just has an email
* ~~MEI FLOWER AND WINTERSWEET BRANCH OF CHINA FLOWER ASSOCIATION - PRUNUS MUME~~
  * Entry just has an email
* CLIVIA SOCIETY
  * HTML table with pagination (https://cliviasociety.com/clivia-register/clivia-register-list/)
* CONNECTICUT AGRICULTURAL EXPERIMENT STATION (CAES)
  * PDF (https://portal.ct.gov/-/media/caes/documents/biographies/anagnostakis/cultivars-of-chestnut.pdf?la=en)
* CONROY, RACHEL COLETTE
  * Website (https://hoyacultivars.org/)
* CYCLAMEN SOCIETY
  * HTML page (https://www.cyclamen.org/plant/cultivar-list/)
* EASTER, MRS MARGARET
  * https://thymus.co.uk/nomenclature1.html
  * Can't find a list but there are multiple pages of cultivars
* DEUTSCHE KAKTEEN-GESELLSCHAFT (GERMAN CACTUS SOCIETY)
  * HTML with JS pagination, each entry links to a page with complete information
  * Lepismium and Rhipsalis are in a PDF (https://www.schlumbergera.net/13770-2/)
* ~~EPIPHYLLUM SOCIETY OF AMERICA (ESA) - Cactaceae Juss. Tribe Hylocereeae~~
  * No print edition available - online edition might appear at some point
* EUROPEAN KALMIA SOCIETY
  * List is here: https://www.kalmia-society.org/en/?job=kalmia
    * Each entry has an `onClick` handler which contains the ID to call https://www.kalmia-society.org/getkalmia.php?i=235&l=en with
* GERANIACEAE GROUP
  * PDFs
* GESNERIAD SOCIETY, INC.
  * HTML with front-end pagination
* GREEN, MR KEITH
  * HTML page https://www.scrapbooklithops.com/cultivars.html
* HAWORTHIA SOCIETY OF JAPAN
  * Multiple addition PDFs, pre-2013 seems to not be available
* HELICONIA SOCIETY INTERNATIONAL - COSTACEAE
  * PDFs
* HELICONIA SOCIETY INTERNATIONAL - HELICONIA
  * PDFs
* ~~HOLLY SOCIETY OF AMERICA~~
  * Can't find anything about the registry on their website
* INDIAN AGRICULTURAL RESEARCH INSTITUTE (IARI) - BOUGAINVILLEA
  * Can't find anything about the registry on their website
* INDIAN AGRICULTURAL RESEARCH INSTITUTE (IARI) - MANGO
  * Can't find anything about the registry on their website
* INTERNATIONAL AROID SOCIETY (I.A.S.)
  * JSON API with Base64-encoded-JSON query parameters: https://www.aroidcultivars.org/_api/cloud-data/v2/items/query?.r=
* INTERNATIONAL CAMELLIA SOCIETY
  * POST request to https://camellia.iflora.cn/Cutivars/SearchListByAllFields
* INTERNATIONAL CARNIVOROUS PLANT SOCIETY (I.C.P.S.)
  * HTML at https://cpnames.carnivorousplants.org/Cultivars.php?name= for each genera
* ~~INTERNATIONAL CULTIVAR REGISTRATION CENTER FOR BAMBOOS (POACEAE, TRIBE BAMBUSEAE)~~
  * Entry just has an email
* ~~INTERNATIONAL CULTIVAR REGISTRATION CENTER FOR OSMANTHUS (ICRCO)~~
  * Website seems to be down
* INTERNATIONAL HIBISCUS SOCIETY
  * Multiple pages https://internationalhibiscussociety.org/searchive/cvindex?start=0&letter=a
* INTERNATIONAL LILAC SOCIETY (USA)
  * Excel file available for members, otherwise PDFs
* ~~INTERNATIONAL MAPLE SOCIETY~~
  * Accepted hybrid list not available yet
* INTERNATIONAL OAK SOCIETY (IOS)
  * Full list and individual information from pages like http://www.oaknames.org/search/fullname.asp?id=2390
* INTERNATIONAL POPLAR COMMISSION, F.A.O.
  * PDF for Salix https://salix-psla.media.uconn.edu/wp-content/uploads/sites/3416/2021/11/CHECKLIST-for-CULTIVARS-of-Salix-L.-willow.pdf
  * Poplars don't seem to be available
* INTERNATIONAL WATERLILY AND WATER GARDENING SOCIETY (IWGS) - NELUMBO
  * List available https://plants.iwgs.org/Home/Search
    * Data is available as a JS object at the bottom of the page
* INTERNATIONAL WATERLILY AND WATER GARDENING SOCIETY (I.W.G.S.) - NYMPHAEACEAE
  * Same as above
* ~~LAKELAND HORTICULTURAL SOCIETY~~
  * Can't find a list on their website
* ~~LYCÉE TECHNIQUE AGRICOLE~~
  * Entry just has an email
* MAGNOLIA SOCIETY INTERNATIONAL
  * PDF
* MECONOPSIS GROUP
  * Linked site appears to be taken over by spammers
  * Link from social media has a table iframed (https://www.themeconopsisgroup.org/templateiframe.asp?fnme=cultivarsregistered)
* ~~MORTON ARBORETUM~~
  * Entry just has an email
* ~~NERINE AND AMARYLLID SOCIETY~~
  * Website appears to be down
* NORTH AMERICAN GLADIOLUS COUNCIL
  * PDF with separate parentage PDF
* PASSIFLORA SOCIETY INTERNATIONAL
  * HTML page (https://passiflorasociety.org/registered-passiflora-cultivars-full-index/)
* ~~PELARGONIUM AND GERANIUM SOCIETY (PAGS)~~
  * Nothing on website about registry
* PLUMERIA SOCIETY OF AMERICA, INC.
  * Downloadable Excel files (https://theplumeriasociety.org/plumeria-registration/registered-plumeria/)
* PROTEACEAE CULTIVAR REGISTRATION AUTHORITY, DEPARTMENT OF AGRICULTURE, FORESTRY AND FISHERIES (SOUTH AFRICA)
  * PDFs from https://www.dalrrd.gov.za/index.php/component/content/article/318-proteaceae-cultivar-registration?catid=19&Itemid=437
* ROYAL GENERAL BULBGROWERS' ASSOCIATION (K.A.V.B.)
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - CLEMATIS
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - CONIFERS
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - DAHLIA
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - DELPHINIUM
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - DIANTHUS
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - LILIUM
  * PDFs
* ROYAL HORTICULTURAL SOCIETY (RHS) - NARCISSUS
  * Sequential IDs, but appears to be alphabetical, so not sure the IDs will be stable
  * HTML table for each ID
* ROYAL HORTICULTURAL SOCIETY (RHS) - ORCHIDS
  * Sequential IDs
  * HTML table for each ID
* ROYAL HORTICULTURAL SOCIETY (RHS) - RHODODENDRON
  * PDFs
* ROYAL NEW ZEALAND INSTITUTE OF HORTICULTURE, INC.
  * Multiple genera
  * PDFs
* ~~SAXIFRAGE~~
  * Entry just has an email
* ~~SOUTH CHINA BOTANICAL GARDENS, CHINESE ACADEMY OF SCIENCES~~
  * Entry just has an email
* THE POLLY HILL ARBORETUM
  * Registry mentioned on the website but no list available
* TRADESCANTIA HUB
  * HTML pages with details
* ~~UNITED STATES NATIONAL ARBORETUM~~
  * Entry just has an email
* WHITTAKER, CAROLE
  * Excel file download
* ~~XIAMEN BOTANICAL GARDEN~~
  * Can't find information about registry on site
* ~~ZHENGZHOU FRUIT RESEARCH INSTITUTE (CAAS)~~
  * Can't find information about registry on site
