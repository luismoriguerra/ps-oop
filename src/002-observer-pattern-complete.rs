trait Observer {
    fn update(&self);
}

trait Subject<'a, T: Observer> {
    fn attach_observer(&mut self, observer: &'a T);
    fn detach_observer(&mut self, observer: &'a T);
    fn notify_observers(&self);
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
}

impl<'a, T: Observer + PartialEq> Subject<'a, T> for FileSubject<'a, T> {
    fn attach_observer(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }
    fn detach_observer(&mut self, observer: &'a T) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
            println!("observer with id: {} has been detached", index);
        }
    }
    fn notify_observers(&self) {
        println!("Notifying observers...");
        for observer in self.observers.iter() {
            observer.update();
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
    println!("creating subject");
    let mut file_subject = FileSubject::new();

    println!("creating observers...");

    let antivirus_observer = ObserverProcess {
        id: 1,
        name: "Antivirus".to_string(),
    };
    let cloudfs_observer = ObserverProcess {
        id: 2,
        name: "CloudFS".to_string(),
    };
    let editor_observer = ObserverProcess {
        id: 3,
        name: "Editor".to_string(),
    };

    file_subject.attach_observer(&antivirus_observer);
    file_subject.attach_observer(&cloudfs_observer);
    file_subject.attach_observer(&editor_observer);

    file_subject.notify_observers();

    file_subject.detach_observer(&cloudfs_observer);

    file_subject.notify_observers();

}
