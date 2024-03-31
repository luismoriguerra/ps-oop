trait Observer {
    fn update(&self);
}

struct FileSubject<'a, T: Observer> {
    observers: Vec<&'a T>,
}

impl<'a, T: Observer + PartialEq> FileSubject<'a, T> {
    fn new() -> FileSubject<'a, T> {
        FileSubject {
            observers: Vec::new(),
        }
    }

    fn attach_observer(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        println!("Notifying observers...");
        for observer in self.observers.iter() {
            observer.update();
        }
    }

    fn detach_observer(&mut self, observer: &'a T) {
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

impl Observer for ObserverProcess {
    fn update(&self) {
        println!(
            "ObserverProcess with id: {} and name: {} has been updated",
            self.id, self.name
        );
    }
}

fn main() {
    let mut file_subject = FileSubject::new();

    let antivirus_observer = ObserverProcess {
        id: 1,
        name: "Antivirus".to_string(),
    };

    let couldfs_observer = ObserverProcess {
        id: 2,
        name: "CouldFS".to_string(),
    };

    file_subject.attach_observer(&antivirus_observer);
    file_subject.attach_observer(&couldfs_observer);

    file_subject.notify_observers();

    file_subject.detach_observer(&antivirus_observer);

    file_subject.notify_observers();
}
