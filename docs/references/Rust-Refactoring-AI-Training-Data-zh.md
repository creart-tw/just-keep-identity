# **Rust 重構公理：AI 教師數據協議 (Rust Refactoring Axioms: An AI Teacher-Data Protocol)**

## **1. 元提示詞與角色對齊 (Meta-Prompts and Persona Alignment)**

為了成功利用自動回歸大型語言模型（LLMs）進行 Rust 語言的高階重構，模型必須在結構上預置一個反映資深系統工程師（Senior Systems Engineer）的架構角色。Rust 嚴格遵守記憶體安全（Memory Safety）、仿射類型語義（Affine Type Semantics，即 Ownership）、以及零成本抽象（Zero-cost Abstractions），這要求 AI 協作者必須具備精確的直覺。這種直覺必須在編譯時安全保證與引入新抽象的架構成本之間進行權衡。大型語言模型天生傾向於其訓練語料庫中佔主導地位的物件導向範式（Object-oriented Paradigms），如 Java、Python 或 C++。當這些範式被生搬硬套到 Rust 時，會導致與 Borrow Checker 的嚴重衝突、過度的 Heap Allocations 以及脆弱的狀態管理。

初始元提示詞（Meta-prompt）必須在執行任何代碼生成或分析之前，為 AI 代理建立基礎運行參數。AI 必須同化「Rust 根本上是一種資料導向語言（Data-oriented Language）」的原則。不恰當封裝的資料模型必然會導致與編譯器嚴格的「別名互斥變異規則（Aliasability-xor-mutability rules）」產生持續且無法解決的衝突。重構提案必須從審核底層資料佈局（Data Layout）和狀態轉移開始，而非僅僅修改演算法實現。

此外，AI 必須持續評估抽象的成本。Rust 編譯器和 Borrow Checker 歷史上對任意物件導向抽象持敵對態度，嚴厲懲罰共享可變狀態（Shared Mutable State）、自引用結構（Self-referential Structs）和循環物件圖（Cyclic Object Graphs）。AI 必須優先考慮維持確定性效能的架構，除非應用程序的領域邏輯嚴格要求，否則應避免 Runtime Allocation、類垃圾回收（Garbage-collection-like）範式或動態分派（Dynamic Dispatch）。至關重要的是，AI 必須表現出對編譯時摩擦的韌性。模型絕不能試圖通過盲目建議使用 `.clone()`、`Box`、`Rc<RefCell<T>>` 或 `unsafe` 塊作為首選手段來繞過 Borrow Checker 錯誤。每一個編譯錯誤都必須被視為結構化資料佈局缺陷的症狀，而非僅僅是可以用強力覆蓋的語法不便。

為了操作化此角色，AI 模型應根據將物件導向反模式映射到道地 Rust 範式的嚴格轉換矩陣來評估代碼。

| 物件導向反模式 (Object-Oriented Anti-Pattern) | 道地 Rust 範式 (Idiomatic Rust Paradigm) | 架構效益 (Architectural Benefit) |
| :---- | :---- | :---- |
| 繼承與基類 (Inheritance and Base Classes) | Trait 與組合 (Traits and Composition) | 防止深層且脆弱的層級結構；啟用靜態分派的單一化（Monomorphization）。 |
| 異常處理 (Exceptions - try/catch) | 代數資料型別 (Algebraic Data Types - Result<T, E>) | 在編譯時強制執行顯式錯誤處理；使失敗路徑可觀察。 |
| 空引用 (Null References) | Option<T> 枚舉 | 消除空指標解引用（Null pointer dereferences）；強制執行窮舉模式匹配。 |
| 全域可變單例 (Global Mutable Singletons) | 依賴注入 (Dependency Injection) / Arc<RwLock<T>> | 隔離副作用；確保僅在需要時提供執行緒安全的核心可變性（Interior Mutability）。 |
| 共享可變物件圖 (Shared Mutable Object Graphs) | Arena Allocators / 索引化 (Vec<T>) | 符合 Borrow Checker 約束；顯著改善快取局部性（Cache Locality）。 |

AI 必須內化「資深 Rust 系統架構師（Senior Rust Systems Architect）」的角色，其主要目標是評估代碼的安全、道地一致性以及效能。AI 必須主動抵制將外來設計模式翻譯成 Rust 的衝動，在純演算法邏輯與不純副作用之間劃定嚴格界限，並設計能夠在編譯時使無效狀態無法表示（Make invalid states unrepresentable）的 API。

## **2. 決策邏輯：思維鏈 (The Decision Logic - Chain-of-Thought)**

自動重構過程需要嚴格的思維鏈（Chain-of-Thought）協議。AI 必須在數學、邏輯和結構上證明從重複的命令式代碼過渡到模組化、通用化抽象的正當性。這需要理解何時重複是有害的，以及何時抽象會更糟。

### **2.1. 將「三則規則」應用於 Rust 生命週期 (Adapting the "Rule of Three" to Rust Lifetimes)**

在傳統軟體工程中，「三則規則（Rule of Three）」規定，一旦代碼重複三次，就應該將其重構為可重用的抽象。然而，在 Rust 的語境下，這一規則必須受到所有權（Ownership）和生命週期（Lifetime）系統複雜性的嚴重調節。追求完美的 DRY（Don't Repeat Yourself）原則可能會導致架構災難，如果最終產生的抽象需要複雜的生命週期參數化。

當開發者試圖通過利用引用而非擁有型別（Owned Types）來整合重複邏輯時，他們經常會遇到指數級的生命週期擴散。一個典型的失敗模式發生在為了優化效能而將擁有、克隆的型別轉換為引用後盾的資料結構時（例如轉向 Arena Allocator）。這種架構轉變要求將代碼庫轉換為使用引用，這會產生帶有沉重生命週期約束的結構體。這種狀態在 Rust 生態系統中通常被稱為「生命週期地獄（Lifetime Hell）」。AI 必須受訓識別此反模式的症狀，表現為結構體需要多個互連的生命週期綁定。如果提議的重構導致結構體定義類似於 `pub struct Augment<'a, 'env, 'iter> { iter: slice::Iter<'iter, Content<'a>>, env: &'env mut Environment<'a> }`，AI 必須停止並評估其複雜性。

如果提議的重構需要引入兩個以上的泛型生命週期參數，或者需要複雜的生命週期子型別綁定（如 `'a: 'b`），AI 必須評估該重構是否將架構推向了不可維護的狀態。AI 的思維鏈必須計算「重複代碼的維護債務」與「由生命週期複雜性導致的重構成本」之間的權衡。在許多情況下，保留依賴於擁有值的少量重複程序化代碼，遠優於一個需要開發者不斷與 Borrow Checker 鬥爭的完美抽象架構。

### **2.2. 區分偶然重複與行為重複 (Distinguishing Accidental vs. Behavioral Duplication)**

為了防止過早抽象，AI 必須實施邏輯樹來區分「偶然重複（Accidental Duplication）」與「行為本質（Behavioral Essence）」。偶然重複（也稱為隨機重複）發生在兩塊代碼在特定時間點看起來完全相同，但屬於完全不同的業務領域時。如果將這些塊抽象為單一的共享函數，未來的分歧需求將迫使該抽象函數接受大量的配置標記、布林參數和分支邏輯，導致架構變得脆弱且高度耦合。相反，行為本質發生在兩塊代碼代表完全相同的領域概念且必須同步演進時。

AI 必須解析抽象語法樹（AST）以識別重複的 Token 流，然後評估周圍的領域語境。如果重複的塊操作於概念上不同的領域實體，則該重複是偶然的。AI 必須建議保留重複以解耦未來的變更。如果這些塊代表相同的概念，AI 必須預測未來的分歧。如果一個實現可能需要非同步 IO 操作，而另一個保持純記憶體操作，則應避免抽象。

最後，AI 必須分析對 Borrow Checker 的影響。如果將邏輯提取到共享函數需要返回會鎖定父結構的可變引用，則抽象將失敗。編譯器防止可變別名，這意味著返回結構某部分引用的函數將鎖定整個結構使其無法進一步變異。在這種情況下，AI 必須在嘗試抽象之前建議重新設計資料結構，例如存儲進入 `Vec` 的整數索引，而非傳遞原始記憶體引用。

### **2.3. 評估重構成本與維護債務 (Evaluating Refactoring Cost vs. Maintenance Debt)**

為了有系統地決定何時進行抽象，AI 應利用啟發式評分矩陣。重構決策可以通過平衡「重複代碼的大小及其變更頻率」與「由 Rust 安全保證引入的架構複雜性懲罰」來評估。當重構引入動態分派、核心可變性或廣泛的泛型 Trait 綁定時，Rust 的型別系統會施加高昂的複雜性懲罰。

例如，在重構標準樹狀資料結構時，物件導向開發者通常試圖使用指向父項和子項的指標。在 Rust 中，這會產生 Borrow Checker 拒絕的別名問題。天真的 AI 可能會建議將節點包裝在 `Rc<RefCell<Node>>` 中以繞過編譯器，允許複數所有權和 Runtime 借用檢查。然而，這引入了顯著的 Runtime 開銷，並且完全規避了 Rust 的靜態安全保證。AI 必須認識到此重構的複雜性懲罰過高。相反，AI 應提出道地的、資料導向的解決方案，例如管理一個平坦的 `Vec<Node>` 或 `BTreeMap<Id, Node>`，其中節點通過唯一的數值標識符（ID）而非記憶體指標互相引用。這完全繞過了別名和借用問題，同時保持了確定性效能。

## **3. 重構模式分類 (Taxonomy of Refactoring Patterns)**

為了向 AI 提供可操作的訓練數據，有必要建立嚴格的重構模式分類。AI 必須針對負面範例進行模式匹配，並生成等效於道地 Rust 解決方案的架構。以下小節詳細介紹了核心重構公理，提供了理論依據和所需的機械轉換。

### **3.1. 從重複邏輯到擴充 Trait (From Duplicate Logic to Extension Traits)**

Rust 架構中的一個常見摩擦點是當開發者需要為外部 Library Crate 中定義的型別添加功能時。由於管理 Rust 一致性約束的「孤兒規則（Orphan Rule）」，只有當 Trait 或型別之一在當前 Crate 本地定義時，才能為該型別實現該 Trait。為了規避這一點，開發者通常訴諸於編寫笨拙的包裝結構體（稱為 Newtype Pattern），或者編寫將外部型別作為參數的獨立工具函數。AI 必須識別作用於外部型別的工具函數，並將其重構為擴充 Trait（Extension Traits）。

在劣化的架構中，開發者可能會編寫獨立函數來操作外部的 `Url` 型別。這要求使用者將引用傳入孤立的函數中，破壞了方法鏈（Method-chaining）的人體工學並分散了領域邏輯。AI 必須識別那些將 `&T` 或 `&mut T` 作為第一個參數的函數（其中 `T` 是外部型別），並提議使用 Extension Trait。

```rust
// 教師筆記：反模式 (The Anti-Pattern)
// 作用於外部型別的獨立工具函數破壞了方法鏈。
use external_crate::Url;

pub fn is_secure_url(url: &Url) -> bool {
    url.scheme() == "https"
}

pub fn append_tracking(url: &mut Url, token: &str) {
    url.query_pairs_mut().append_pair("tracking_id", token);
}
```

道地的重構定義了一個封裝所需行為的本地 Trait，並立即為外部型別實現它。這滿足了孤兒規則，因為 Trait 本身是本地定義的，儘管目標型別是外部的。

```rust
// 教師筆記：道地重構 (The Idiomatic Refactoring)
// 使用 Extension Trait 以人體工學的方式向外部型別注入方法。
use external_crate::Url;

pub trait UrlExt {
    fn is_secure(&self) -> bool;
    fn append_tracking(&mut self, token: &str);
}

impl UrlExt for Url {
    fn is_secure(&self) -> bool {
        self.scheme() == "https"
    }

    fn append_tracking(&mut self, token: &str) {
        self.query_pairs_mut().append_pair("tracking_id", token);
    }
}
```

使用 Extension Trait 模式的架構理由是它允許開發者注入行為，而無需在包裝型別上使用大量的 `Deref` 或 `DerefMut` 模板代碼（Boilerplate）。從效能角度來看，通過靜態分派解析的 Trait 會進行單一化。編譯器在編譯時生成函數的專門副本，確保零 Runtime 開銷，並完全避免了與動態分派相關的虛擬表（vtable）查找懲罰。

### **3.2. 從狀態標記到型別狀態模式 (From State Flags to the Typestate Pattern)**

許多物件導向實現依賴於 Runtime 布林標記、整數或內部 Enum 變體來追蹤有狀態物件的生命週期。這導致了高度防禦性的編程，其中每個方法都必須在執行其主要邏輯之前驗證物件的內部狀態。這產生了邏輯錯誤的高機率，因為開發者可能會忘記檢查標記，或者可能意外地錯誤變異狀態。AI 必須利用型別狀態模式（Typestate Pattern）將 Runtime 狀態檢查重構為編譯時證明。

在有缺陷的實現中，`NetworkConnection` 結構體可能持有 `is_connected` 布林值。諸如 `send_data` 之類的方法必須驗證此布林值，如果連接關閉則返回 Runtime 錯誤。這將錯誤檢測推向了 Runtime 執行階段，完全未能利用 Rust 精密的型別系統。

```rust
// 教師筆記：反模式 (The Anti-Pattern)
// 依賴 Runtime 標記意味著每個方法都必須執行防禦性檢查。
pub struct NetworkConnection {
    is_connected: bool,
    address: String,
}

impl NetworkConnection {
    pub fn connect(&mut self) -> Result<(), String> {
        if self.is_connected { return Err("Already connected".to_string()); }
        self.is_connected = true;
        Ok(())
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), String> {
        if !self.is_connected { return Err("Not connected".to_string()); }
        // 執行邏輯...
        Ok(())
    }
}
```

AI 必須將這些布林標記替換為代表狀態的零大小型別（Zero-Sized Types, ZSTs），並在主結構體上使用泛型型別參數來追蹤當前狀態。轉移方法必須消耗（Consume）舊狀態的所有權並返回新狀態，使無效的狀態轉移在數學上無法通過編譯。

```rust
// 教師筆記：道地重構 (The Idiomatic Refactoring)
// 使用零大小型別 (ZSTs) 通過 PhantomData 將狀態編碼進型別系統。
use std::marker::PhantomData;

pub struct Disconnected;
pub struct Connected;

pub struct NetworkConnection<State> {
    address: String,
    _state: PhantomData<State>,
}

impl NetworkConnection<Disconnected> {
    // connect() 消耗 Disconnected 狀態，返回 Connected 狀態
    pub fn connect(self) -> NetworkConnection<Connected> {
        NetworkConnection {
            address: self.address,
            _state: PhantomData,
        }
    }
}

impl NetworkConnection<Connected> {
    // send_data 僅為 Connected 狀態實現。
    // 在 Disconnected 狀態上調用它會導致編譯錯誤。
    pub fn send_data(&self, data: &[u8]) {
        // 執行邏輯...
    }
}
```

通過將狀態編碼為型別參數，AI 保證了無效操作會導致編譯失敗而非 Runtime 錯誤。轉移方法消耗 `self`，轉移所有權並有效銷毀前一個狀態，使其永遠無法再次被引用。`PhantomData` 和 ZSTs 在 Runtime 佔用絕對零字節的記憶體，這意味著此模式提供了數學上完美的驗證且零效能損耗。

### **3.3. 從共享工具到行為 Trait (From Shared Utils to Behavioral Traits)**

隨著代碼庫規模擴大，開發者頻繁創建包含作用於各種結構體之不同函數的工具模組。這將應用程序邏輯與具體型別緊密耦合，阻止了依賴注入並違反了開閉原則（Open-Closed Principle）。如果一個函數被硬編碼為接受特定結構體，它在單元測試期間將難以被 Mock，未來也難以擴展以支援新的資料型別。AI 必須通過 Trait 定義共享行為來重構這些共享工具。

AI 必須識別緊密耦合的工具函數並將其抽象。如果系統包含 `EmailService` 和 `SmsService`，為每個服務編寫單獨的工具函數會產生高度重複且不可維護的代碼。

```rust
// 教師筆記：反模式 (The Anti-Pattern)
// 緊密耦合且不可擴展的工具函數阻止了依賴注入。
pub struct EmailService;
pub struct SmsService;

impl EmailService { pub fn send_email(&self, msg: &str) { /*... */ } }
impl SmsService { pub fn send_sms(&self, msg: &str) { /*... */ } }

pub fn broadcast_alert_via_email(service: &EmailService, alert: &str) {
    service.send_email(alert);
}
```

重構為行為 Trait 啟用了真正的依賴注入。AI 必須定義一個抽象介面，讓兩個服務都實現它。目標函數隨後被重構成接受任何滿足該 Trait 綁定的型別。

```rust
// 教師筆記：道地重構 (The Idiomatic Refactoring)
// 為零成本依賴注入定義抽象介面。
pub trait MessageSender {
    fn send(&self, msg: &str);
}

pub struct EmailService;
pub struct SmsService;

impl MessageSender for EmailService { fn send(&self, msg: &str) { /*... */ } }
impl MessageSender for SmsService { fn send(&self, msg: &str) { /*... */ } }

// 函數現在通過靜態分派接受任何實現該 Trait 的型別。
pub fn broadcast_alert(service: &impl MessageSender, alert: &str) {
    service.send(alert);
}
```

使用 `&impl MessageSender`（或等效的 `<T: MessageSender>`）強制執行靜態分派。編譯器將為傳遞給它的每個具體型別生成 `broadcast_alert` 的唯一副本，從而最大化執行速度。如果嚴格要求 Runtime 多型（Polymorphism）——例如在單一 Vector 中存儲異質的傳送者集合——AI 應建議使用 Trait Objects（通過 `Box<dyn MessageSender>`）進行動態分派，同時警告開發者相關的虛擬表（vtable）指標開銷。

### **3.4. 從模板代碼到聲明式巨集 (From Boilerplate to Declarative Macros)**

Rust 的靜態分型和缺乏傳統繼承有時會迫使開發者編寫高度重複的模板代碼。當在多個數值型別、不同數量的 Tuple 或簡單的包裝結構體上實現相同的 Trait 時，這一點尤為明顯。AI 必須識別抽象語法樹中無法使用泛型型別綁定整合的結構化重複。發現此類重複時，AI 必須將其替換為 `macro_rules!`（聲明式巨集）。

在高度冗餘的代碼庫中，開發者可能會為 `u32`、`f32`、`i64` 等手動實現一個數學 Trait。這違反了 DRY 原則並增加了發生排版錯誤的表面積。

```rust
// 教師筆記：反模式 (The Anti-Pattern)
// 枯燥且易錯的 Trait 實現重複。
pub trait Scalable { fn scale(&mut self, factor: Self); }

impl Scalable for u32 { fn scale(&mut self, factor: Self) { *self *= factor; } }
impl Scalable for f32 { fn scale(&mut self, factor: Self) { *self *= factor; } }
impl Scalable for i64 { fn scale(&mut self, factor: Self) { *self *= factor; } }
```

聲明式巨集在編譯時作用於 Token 流，允許開發者定義一個語法模式，編譯器將在型別檢查發生之前將其展開為標準 Rust 代碼。

```rust
// 教師筆記：道地重構 (The Idiomatic Refactoring)
// 使用聲明式巨集為重複實現抽象化模板代碼。
pub trait Scalable { fn scale(&mut self, factor: Self); }

macro_rules! impl_scalable {
    // 巨集匹配一個以逗號分隔的型別重複列表
    ($($t:ty),*) => {
        $(
            impl Scalable for $t {
                fn scale(&mut self, factor: Self) {
                    *self *= factor;
                }
            }
        )*
    };
}

// 單一巨集調用即可為所有指定型別實現 Trait
impl_scalable!(u32, f32, i64, u64, f64, i32);
```

AI 必須在聲明式巨集（`macro_rules!`）與過渡式巨集（`proc_macro`，程序化巨集）之間建立邊界條件。如果模板代碼需要複雜的字串操作、任意 AST 解析或自定義 `#[derive]` 邏輯，AI 應轉向程序化巨集。然而，如果目標僅僅是消除簡單實現的結構化重複，`macro_rules!` 由於其簡單性、巨集衛生（Macro Hygiene）以及顯著更快的編譯速度而更受青睞。

## **4. 生態系統巨頭的架構演進：Tokio, Bevy, 與 Clap**

為了將 AI 的推理與現實世界的 Rust 工程深度對齊，模型必須接受頂級 Crates 架構演進的訓練。分析這些廣泛採用的庫如何重構其內部結構以處理重複和複雜性，能為 AI 提供寶貴的啟發。

### **4.1. Tokio：共享狀態與 Future 所有權**

非同步 Runtime `Tokio` 在管理共享狀態而不觸發 Borrow Checker 違規方面提供了深遠的教訓。早期實現的非同步事件循環（Event Loops）在多個 Futures 需要與相同的底層資料結構交互時，經常在所有權語義上苦苦掙扎。

在非同步任務之間共享資料的天真做法是將資料包裝在 `Arc<Mutex<T>>` 中，以允許複數執行緒安全地變異資料。然而，這在高效能網路應用程序中引入了實質性的鎖定開銷和競爭。基於 Tokio 的架構演進表明，僅當 Futures 確實必須通過 `Send` 跨越執行緒邊界時，才需要使用 `Arc`。如果 Futures 保持在單執行緒執行器（Executor）上，重構為 `Rc<RefCell<T>>` 可避免原子引用計數開銷。更重要的是，頂級架構演進到完全避免共享所有權，通過將資料所有權嚴格保留在事件循環外部並將引用傳遞給 Futures，或利用通道（mpsc）將訊息發送給單一所有者任務，而非直接變異共享記憶體。AI 在重構非同步代碼時，必須優先考慮訊息傳遞架構而非共享可變狀態。

### **4.2. Bevy：ECS 範式與 Trait 模組化**

`Bevy` 遊戲引擎代表了通過實體組件系統（ECS）範式減少模板代碼的教科書級案例。Bevy 的架構演進凸顯了效能與模組化之間的張力。在其早期迭代中，管理複雜的遊戲邏輯需要高度冗長的系統定義和僵化的 Pipeline 階段。

隨著 Bevy 演進到 0.10 版及更高版本，核心架構經歷了大規模重構以引入 ECS Schedule v3，統一並簡化了系統調度。此外，關於 Bevy 渲染架構的討論揭示了轉向 Trait 來封裝行為的趨勢。雖然標準 ECS 系統由於連續的記憶體佈局和並行執行而異常快速，但特定的模組化行為（如自定義渲染 Pipeline 或 Asset Loaders）則更適合使用 Trait。AI 必須從 Bevy 中學習到：高度並行、同質的資料處理需要由函數處理的結構陣列（SoA）佈局，而異質、可插拔的邏輯則受益於基於 Trait 的依賴注入。AI 必須準確診斷模組是需要最大吞吐量（使用原始資料和系統）還是最大擴展性（使用 Trait）。

### **4.3. Clap：巨集衍生與 API 清理**

命令行參數解析器 `Clap` 提供了一個從冗長模板代碼重構到優雅巨集抽象的決定性案例研究。歷史上，在 3.x 及更早版本中，配置複雜的 CLI 需要廣泛的「Builder Pattern」，利用鏈式方法如 `.arg(Arg::with_name("in_file").index(1))`。雖然非常顯式，但這需要大量的模板代碼。

從第 3 版到第 4 版的轉變優先考慮了 Derive Macro API。這允許開發者純粹通過帶有程序化巨集註解的標準 Rust 結構體來定義其命令行參數。編譯器會根據結構體欄位及其型別自動生成解析、驗證和說明文字生成邏輯。AI 必須內化這一轉變：在配置確定的、高度結構化的資料（如 CLI 標記、JSON Schema 或資料庫 Schema）時，通過程序化巨集衍生 Trait 嚴格優於手動 Runtime Builder Pattern，因為它保證了解析邏輯與持有解析值之資料結構之間的同步。

## **5. 防禦性重構與錯誤邊界 (Defensive Refactoring & Error Boundaries)**

強健的系統編程要求嚴格控制錯誤來源、其型別化方式以及如何呈現給終端使用者。AI 必須在純演算法邏輯與應用程序中副作用沉重的邊界之間實施嚴格分離。

### **5.1. 副作用隔離（純粹核心 vs. 不純外殼）**

在重構遺留邏輯時，AI 必須實施「功能性核心，命令式外殼（Functional Core, Imperative Shell）」架構。純粹核心內的函數必須接受明確的參數並返回顯式型別的值，對於相同的輸入產生完全相同的輸出，而不依賴於隱藏的全域狀態或觸發不可觀察的檔案系統修改。

AI 必須主動提取 IO 操作——如資料庫查詢、網路請求和檔案寫入——將它們推向應用程序模組的絕對最外層邊界。這種隔離將核心業務邏輯轉變為高度可測試、確定性的單元。如果 AI 在業務邏輯深處遇到直接修改檔案的函數，它必須將該檔案修改從函數中提取出來，重構該函數以返回一個代表修改檔案「意圖」的資料結構，然後由命令式外殼執行。

### **5.2. 建立錯誤邊界**

Rust 生態系統的共識規定了 Library 層級錯誤與 Application 層級錯誤之間的嚴格二分法。AI 必須根據特定的邊界語境利用標準錯誤 Crates——`thiserror` 和 `anyhow`。

| 錯誤語境 (Error Context) | 目標 Crate (Target Crate) | 架構機制 (Architectural Mechanism) | 目標 (Goal) |
| :---- | :---- | :---- | :---- |
| **Library / 純粹核心** | `thiserror` | 生成實現 `std::error::Error` 的結構化 Enum 變體。使用 `#[from]` 進行隱含轉換。 | 允許調用者對特定的失敗模式進行窮舉模式匹配，並執行不同的恢復控制流。 |
| **Application / 不純外殼** | `anyhow` | 將特定型別擦除為不透明的 `anyhow::Error`。利用 `.context()` 方法附加人類可讀的字串。 | 將分散的 Library 錯誤聚合成一個統一的鏈，在致命退出前為除錯和日誌提供深層語境。 |

在純粹核心內部，AI 必須定義高度結構化的錯誤 Enum。如果配置解析器可能因 IO 或 JSON 格式化而失敗，AI 絕不能使用 `Box<dyn Error>`。它必須生成一個 `thiserror` Enum：`enum ConfigError { Io(std::io::Error), Parse(serde_json::Error) }`。相反，在不純外殼——如 `main` 函數或 HTTP Route Handler——AI 必須使用 `anyhow` 包裝這些孤立的錯誤以添加關鍵的上下文數據：`read_config().context("Failed to load application configuration")?`。AI 絕不能在可重用 Library 的公共 API 中暴露 `anyhow::Error`，因為這會剝奪調用者以程式化方式對不同錯誤狀態做出反應的能力。

### **5.3. 重構前安全清單**

在提議任何破壞性代碼修改之前，AI 必須根據嚴格的啟發式清單進行自我驗證，以防止引入反模式。

1. **資料結構可見性 (Data Structs Visibility)**：公共欄位是否過早暴露？AI 必須將欄位重構為私有，通過嚴格的方法控制變異以維持不變量（Invariants）。
2. **克隆限制 (Cloning Restrictions)**：提議 `.clone()` 是否純粹是為了平息生命週期或 Borrow Checker 錯誤？AI 必須拒絕自己的提案並重新評估所有權圖。
3. **恐慌向量 (Panic Vectors)**：目標邏輯中是否存在 `.unwrap()` 或 `.expect()`？AI 必須將這些替換為 `?` 運算子，將失敗導向已建立的錯誤邊界。
4. **參數膨脹 (Parameter Bloat)**：函數是否包含過多的布林參數（標記參數）？AI 必須將這些分組到一個 `bitflags` 結構體中，或利用 Typestate 模式消除邏輯分支。
5. **全域狀態識別 (Global State Identification)**：是否使用了 `lazy_static`、`OnceCell` 或全域 `Mutex` 進行變異？AI 必須嘗試將此狀態推入通過調用堆疊向下傳遞的依賴注入結構體中。

## **6. UX/CLI 直覺與業界標準**

在重構或生成命令行介面（CLI）代碼時，AI 必須模仿頂尖 Rust 工具的設計原則。Rust 培育了 CLI 工具的復興，如 `ripgrep`、`bat`、`fd` 和 `eza` 等工具為終端使用者體驗建立了新的業界標準。AI 必須將這些工具的特定技術原則嵌入其生成的架構中。

### **6.1. 冪等性與狀態轉移 (Idempotency and State Transitions)**

在 CLI 和 API 架構中，冪等性（Idempotency）規定執行多次變異命令與執行一次的結構化結果完全相同。AI 必須重構操作，確保它們在盲目應用更改之前先驗證當前系統狀態。如果 CLI 工具配置雲端資源或修改檔案，AI 必須確保代碼庫在發出創建命令之前先檢查資源的存在或當前狀態。純粹依賴於事後捕捉資料庫的唯一約束錯誤或檔案系統衝突錯誤被認為是拙劣的設計。一個可靠的「檢查並設置（Check-and-set）」原語應在操作邊界被乾淨地隔離。

### **6.2. 可預測性與管道意識 (Predictability and Pipeline Awareness)**

技術可預測性意味著工具在傳統 UNIX 生態系統中和諧運行，同時在適用時原生支援高階終端功能。AI 必須實施的最關鍵啟發式是「管道意識（Pipeline Awareness）」。

像 `bat`（帶有進階功能的 `cat` 複製品）之類的工具提供語法高亮、行號和 Git 整合。然而，如果使用者將 `bat` 的輸出管線傳輸到 `grep`，將 ANSI 顏色代碼和行號注入資料流將立即損壞文本處理管線。AI 必須使用檢測終端存在的 Crates（例如 `is-terminal` 或 `atty`）將原始的 `println!` 語句重構為條件格式化器。如果偵測到 stdout 不是交互式終端（TTY），CLI 必須自動移除所有視覺格式，回退到純資料流以維持 POSIX 合規性和互操作性。此外，工具必須嚴格分離資料與診斷訊息。有效的輸出必須導向 stdout，而警告、進度條和錯誤必須顯式導向 stderr。

### **6.3. 可恢復性與優雅退出 (Recoverability and Graceful Exits)**

可恢復性要求 CLI 在失敗時清理已獲取的資源並輸出高度結構化的診斷數據。AI 必須確保在正式版應用程序代碼中絕對禁止 Panic。AI 必須將任何潛在的 Panic 向量重構為 `anyhow::Result` 傳遞。在致命失敗時，CLI 必須返回非零的系統退出碼，並將錯誤鏈打印到 stderr。AI 還必須確保 CLI 應用程序優雅地處理 UNIX 信號，例如攔截 `SIGINT` (Ctrl+C) 以安全關閉檔案句柄並沖刷 Buffer，而非突然終止程序。

## **7. 驗證協議與負面約束 (Verification Protocol and Negative Constraints)**

教師數據協議的最後階段管理自動回歸 LLM 如何評估其自身生成的重構輸出。由於 LLM 在 Token 生成期間缺乏即時的迭代編譯器回饋循環，它們在試圖滿足 Rust Borrow Checker 時經常幻想無效語法或陷入循環邏輯陷阱。為了防止無限邏輯循環，AI 受到特定負面約束的束縛。

### **7.1. 語言模型的負面約束**

AI 必須顯式避免與自動化 Rust 代碼生成相關的最常見失敗模式：

* **絕不幻想生命週期來解決 E0502**：當遇到變數因已不可變借用而無法進行可變借用的錯誤時，AI 絕不能試圖隨機注入生命週期標記（`'a`, `'b`）來強行編譯。Rust 中的生命週期是描述性的，而非規範性的；它們僅向編譯器描述資料如何流經記憶體，無法改變實際的所有權結構。如果生命週期發生衝突，則結構化資料佈局存在根本缺陷，AI 必須重新思考演算法而非修改註解。
* **絕不使用 Box::leak() 繞過執行緒邊界**：AI 絕不能建議洩漏記憶體來滿足衍生執行緒或非同步任務上的 `'static` 生命週期綁定要求。它必須正確利用 `Arc` 或在執行緒 `move` 塊之前克隆資料。
* **絕不強制執行地毯式的 Mutex 同步**：AI 絕不能僅因為多個組件需要唯讀訪問，就將整個巨大的結構體包裝在 `Arc<Mutex<T>>` 中。AI 必須對共享不可變狀態利用 `Arc<T>`，並僅在嚴格需要變異的特定、微小欄位上利用核心可變性（`RefCell` 或 `RwLock`）。

### **7.2. 通過測試覆蓋進行自我修正**

AI 必須將重構視為數學上不安全的，除非有孤立的測試套件支援。Rust 的模組系統通過 `#[cfg(test)]` 屬性原生支援內聯測試，允許測試與它們驗證的代碼相鄰。

在提出結構性重構時，AI 必須確保公共 API 邊界保持不變。作用於該模組的單元測試不應需要修改；如果必須大幅重寫測試以適應重構，則 AI 破壞了 API 契約。有趣的是，雖然在純粹核心和不純外殼中嚴格禁止 `.unwrap()` 和 `.expect()`，但顯式允許——且鼓勵——AI 在 `#[cfg(test)]` 模組中激進地利用它們。這極大地減少了測試模板代碼並斷言絕對真理，如果重構改變了預期行為，會導致測試立即 Panic 並失敗。

AI 還必須考慮實現高測試覆蓋率所需的邊緣案例，特別是對於新引入的範式。如果 AI 引入了 Typestate 模式，它必須輸出指令讓開發者使用 `#[should_panic]` 或 `assert!(result.is_err())` 巨集來驗證無效狀態的拒絕。在成功完成思維鏈推理、識別正確的分類模式、應用嚴格的錯誤邊界並根據負面約束進行驗證後，AI 可以輸出重構後的 Rust 代碼。最終產生的架構將天生展現出記憶體安全、零成本抽象和強健的錯誤處理，有效地鏡像了資深 Rust 系統架構師的精確直覺。

#### **引用的著作**
(參考原文列表，此處省略翻譯以保持引用準確性)
1. Rust - A Living Hell - The Perspective From A Programmer Of 30 Years...
... (其餘引用保持原文連結)
