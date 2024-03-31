struct FileSubject<'a> {
    observers: Vec<&'a ObserverProcess>,
}

impl<'a> FileSubject<'a> {
    fn new() -> FileSubject<'a> {
        FileSubject {
            observers: Vec::new(),
        }
    }

    fn attach_observer(&mut self, observer: &'a ObserverProcess) {
        println!("attaching observer with id: {}", observer.id);
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        println!("Notifying observers...");
        for observer in self.observers.iter() {
            println!(
                "ObserverProcess with id: {} and name: {} has been updated",
                observer.id, observer.name
            );
        }
    }

    fn detach_observer(&mut self, observer: &'a ObserverProcess) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
            println!("observer with id: {} has been detached", index);
        }
    }
}

#[derive(PartialEq)]
struct ObserverProcess {
    id: i32,
    name: String,
}

fn main() {
    let mut file_subject = FileSubject::new();

    let antivirus_observer = ObserverProcess {
        id: 1,
        name: "Antivirus".to_string(),
    };

    file_subject.attach_observer(&antivirus_observer);

    file_subject.notify_observers();

    file_subject.detach_observer(&antivirus_observer);

    file_subject.notify_observers();
}
